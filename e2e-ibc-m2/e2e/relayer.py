from subprocess import Popen
import logging as l

from .cmd import Config


def start(c: Config) -> Popen:
    full_cmd = f'{c.relayer_cmd} -c {c.config_file} -j start'.split(' ')
    # l.debug(' '.join(full_cmd))
    print()
    print('\033[32;1m{msg}\033[0m'.format(msg=' '.join(full_cmd)))
    print()
    return Popen(full_cmd)
