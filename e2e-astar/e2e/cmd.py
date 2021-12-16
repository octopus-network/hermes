#!/usr/bin/env python3

import json
import logging as l
import subprocess
from dataclasses import dataclass, fields as datafields, is_dataclass
from pathlib import Path
from typing import Any, List, TypeVar, Generic, Type, Callable


@dataclass
class Config:
    config_file: Path
    relayer_cmd: str
    log_level: str
    max_retries: int = 10


T = TypeVar('T')


@dataclass
class CmdResult(Generic[T]):
    cmd: 'Cmd'
    config: Config
    result: Any
    retries: int = 0

    def success(self) -> T:
        status = self.result.get('status') or 'unknown'
        result = self.result.get('result') or {}

        if status == "success":
            data = self.cmd.process(result)
            # l.debug(str(data))
            return data
        elif self.retries < self.config.max_retries:
            left = self.config.max_retries - self.retries
            # l.warn(f'Command failed: retrying (retries left: {left})')
            print('\033[33;1m{msg}\033[0m'.format(
                msg="Command failed: retrying ...(retries left: " + str(left) +
                ")"))
            return self.cmd.retry(self.config, self.retries).success()
        else:
            raise ExpectedSuccess(self.cmd, status, result)


class Cmd(Generic[T]):
    name: str

    def process(self, result: Any) -> Any:
        raise NotImplementedError("Cmd::process")

    def args(self) -> List[str]:
        raise NotImplementedError("Cmd::args")

    def to_cmd(self) -> str:
        return f"{self.name} {' '.join(self.args())}"

    def run(self, config: Config, retries: int = 0) -> CmdResult[T]:
        full_cmd = f'{config.relayer_cmd} -c {config.config_file} -j'.split(' ')
        full_cmd.extend(self.name.split(' '))
        full_cmd.extend(self.args())
        cmd = ' '.join(full_cmd)
        print()
        print('\033[32;1m{msg}\033[0m'.format(msg=cmd))
        print()
        # l.debug(cmd)

        ret = subprocess.check_output(cmd,
                                      text=True,
                                      shell=True,
                                      encoding="utf-8")
        # l.debug(ret)
        js = json.loads(ret)
        print('\033[36;1m{msg}\033[0m'.format(msg=json.dumps(js, indent=2)))

        return CmdResult(cmd=self, config=config, retries=retries, result=js)

    def retry(self, config: Config, retries: int) -> CmdResult[T]:
        return self.run(config, retries + 1)


C = TypeVar('C', bound=Cmd)


def cmd(name: str) -> Callable[[Type[C]], Type[C]]:

    def decorator(klass: Type[C]) -> Type[C]:
        klass.name = name
        return klass

    return decorator


def from_dict(klass, dikt) -> Any:
    if is_dataclass(klass):
        fields = datafields(klass)
        args = {f.name: from_dict(f.type, dikt[f.name]) for f in fields}
        return klass(**args)
    else:
        return dikt


class ExpectedSuccess(Exception):
    cmd: Any
    status: str
    result: Any

    def __init__(self, cmd: Any, status: str, result: Any) -> None:
        self.cmd = cmd
        self.status = status
        self.result = result

        super().__init__(
            f"\033[31;1m Command '{cmd}' failed. Expected 'success', got '{status}'. Message: {result} \033[0m"
        )
