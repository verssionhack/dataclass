use std::{cell::{Cell, RefCell}, sync::{RwLock, Mutex}};

use proc_dataclass::Dataclass;





#[derive(Debug, Dataclass, Default)]
struct TestStruct {
    #[dataclass(setter(skip))]
    _name_: String,
    #[dataclass(getter(name="get_age_cell"), setter(name="set_age_cell", pub_scope="crate"))]
    age_cell: Cell<u16>,
    #[dataclass(getter(name="get_age_ref_cell"), setter(name="set_age_ref_cell"))]
    age_ref_cell: RefCell<u16>,
    #[dataclass(getter(name="get_age_rw"), setter(name="set_age_rw"))]
    age_rw: RwLock<u16>,
    #[dataclass(getter(name="get_age_lock"), setter(name="set_age_lock", pub_scope="self"))]
    age_lock: Mutex<u16>,
}

#[derive(Debug, Dataclass, Default)]
struct TestVarint(
    #[dataclass(getter(name="get_name"), setter(name="set_name"))]
    String,
    #[dataclass(getter(name="get_age_cell"), setter(name="set_age_cell"))]
    Cell<u16>,
    #[dataclass(getter(name="get_age_ref_cell"), setter(name="set_age_ref_cell"))]
    RefCell<u16>,
    #[dataclass(getter(name="get_age_rw"), setter(name="set_age_rw"))]
    RwLock<u16>,
    #[dataclass(getter(name="get_age_lock"), setter(name="set_age_lock"))]
    Mutex<u16>,
    #[dataclass(getter(skip), setter(skip))]
    Mutex<u16>,
    #[dataclass(skip)]
    Mutex<u16>,
);



fn main() {
    let mut test_struct = TestStruct {
        _name_: "yinpeach".to_string(),
        ..Default::default()
    };
    println!("name: {}", test_struct.get_name());
    let test_varint = TestVarint::default();
}

