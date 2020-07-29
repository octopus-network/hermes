use std::collections::HashMap;
use std::fmt::Display;

use serde; // 1.0.114
use serde_json; // 1.0.56

use serde::{de::DeserializeOwned, Serialize};

trait Path: Display {
    type Value: Serialize + DeserializeOwned;
}

trait Store {
    fn get<P: Path>(&self, path: &P) -> Option<P::Value>;

    fn set<P: Path>(&mut self, path: &P, value: P::Value);

    fn contains<P: Path>(&self, path: &P) -> bool;
}

struct MapStore {
    map: HashMap<String, String>,
}

impl MapStore {
    fn get_raw<P: Path>(&self, path: &P) -> Option<String> {
        self.map.get(&path.to_string()).cloned()
    }
}

impl Store for MapStore {
    fn get<P: Path>(&self, path: &P) -> Option<P::Value> {
        let json = self.map.get(&path.to_string())?;
        serde_json::from_str(json).ok()
    }

    fn set<P: Path>(&mut self, path: &P, value: P::Value) {
        self.map
            .insert(path.to_string(), serde_json::to_string(&value).unwrap());
    }

    fn contains<P: Path>(&self, path: &P) -> bool {
        self.map.contains_key(&path.to_string())
    }
}

enum Op {
    Set {
        path: String,
        value: String,
    },
    Get {
        path: String,
        result: Option<String>,
    },
    Contains {
        path: String,
        result: bool,
    },
}

use std::cell::RefCell;

struct LogStore {
    map: MapStore,
    log: RefCell<Vec<Op>>,
}

impl Store for LogStore {
    fn get<P: Path>(&self, path: &P) -> Option<P::Value> {
        self.log.borrow_mut().push(Op::Get {
            path: path.to_string(),
            result: self.map.get_raw(path),
        });

        self.map.get(path)
    }

    fn set<P: Path>(&mut self, path: &P, value: P::Value) {
        self.log.borrow_mut().push(Op::Set {
            path: path.to_string(),
            value: serde_json::to_string(&value).unwrap(),
        });

        self.map.set(path, value);
    }

    fn contains<P: Path>(&self, path: &P) -> bool {
        let result = self.map.contains(path);

        self.log.borrow_mut().push(Op::Contains {
            path: path.to_string(),
            result,
        });

        result
    }
}

fn main() {}
