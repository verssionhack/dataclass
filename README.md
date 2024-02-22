# A Simple Rust DataClass ProdureMacro
---------------------------------------

## Goals

Auto implement setter and getter method for dataclass

## Import
To follow line in your Cargo.toml
```
[dependencies]
dataclass = { git = "https://github.com/verssionhack/dataclass.git" }
```

## Usage

Say this is a Struct named TestStruct and a Varint named TestVarint:
```
#[derive(Dataclass)]
struct TestStruct {
    #[dataclass(getter(name="get_name"), setter(name="set_name"))]
    _name_: String,
    #[dataclass(getter(name="get_age_cell"), setter(name="set_age_cell", pub_scope="crate"))]
    age_cell: Cell<u16>,
    #[dataclass(getter(name="get_age_ref_cell"), setter(name="set_age_ref_cell"))]
    age_ref_cell: RefCell<u16>,
    #[dataclass(getter(name="get_age_rw"), setter(name="set_age_rw"))]
    age_rw: RwLock<u16>,
    #[dataclass(getter(name="get_age_lock"), setter(name="set_age_lock", pub_scope="self"))]
    age_lock: Mutex<u16>,
};

#[derive(Dataclass)]
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
)
```
Both has a derive: #[derive(Dataclass)] and Some field has attr #[dataclass]

Which will auto impl setter and getter like:

```
impl TestStruct {
    pub fn get_name(&self) -> &str {
        &self._name_
    }
    pub fn set_name(&mut self, value: String) {
        self._name_ = value;
    }
    pub fn get_age_cell(&self) -> u16 {
        self.age_cell.get()
    }
    pub(crate) fn set_age_cell(&self, value: u16) {
        self.age_cell.set(value);
    }
    pub fn get_age_ref_cell(&self) -> u16 {
        *self.age_ref_cell.borrow()
    }
    pub fn set_age_ref_cell(&self, value: u16) {
        *self.age_ref_cell.borrow_mut() = value;
    }
    pub fn get_age_rw(&self) -> u16 {
        *self.age_rw.read().unwrap()
    }
    pub fn set_age_rw(&self, value: u16) {
        *self.age_rw.write().unwrap() = value;
    }
    pub fn get_age_lock(&self) -> u16 {
        *self.age_lock.lock().unwrap()
    }
    pub(self) fn set_age_lock(&self, value: u16) {
        *self.age_lock.lock().unwrap() = value;
    }
}
impl TestVarint {
    pub fn get_name(&self) -> &str {
        &self.0
    }
    pub fn set_name(&mut self, value: String) {
        self.0 = value;
    }
    pub fn get_age_cell(&self) -> u16 {
        self.1.get()
    }
    pub fn set_age_cell(&self, value: u16) {
        self.1.set(value);
    }
    pub fn get_age_ref_cell(&self) -> u16 {
        *self.2.borrow()
    }
    pub fn set_age_ref_cell(&self, value: u16) {
        *self.2.borrow_mut() = value;
    }
    pub fn get_age_rw(&self) -> u16 {
        *self.3.read().unwrap()
    }
    pub fn set_age_rw(&self, value: u16) {
        *self.3.write().unwrap() = value;
    }
    pub fn get_age_lock(&self) -> u16 {
        *self.4.lock().unwrap()
    }
    pub fn set_age_lock(&self, value: u16) {
        *self.4.lock().unwrap() = value;
    }
}
```

## Field Attribute #[dataclass] Options

There are some available option in field attribute #[dataclass]

## To epecial the method name of getter and setter

##[dataclass(setter(name="your_method_name"), setter(name="your_method_name"))]

## To epecial Pub Scope of getter and setter

scope is one of ["crate", "super", "self"]

##[dataclass(setter(pub_scope="scope"))]

## To disable pub of getter and setter

The default scope of them is pub.
If you want disable it, you can set the pub equals false like #[dataclass(getter(pub=false))]

## To set method to const

#[dataclass(getter(const), setter(const))]

## To skip implement a field

you can skip setter or getter, or both, like

#[dataclass(setter(skip), getter(skip))]
or
#[dataclass(skip)]

#Usage Notice

Default implement for named field will remote leading '_' and trailing '_'

named field name like "_name_" in method will be "get_name" or "set_name",
so if in a struct have two fields named "name" and "_name" or "name_",
the method implement for them will be duplicate.

For field wrapped by one of ["std::sync::RwLock", "std::cell::Cell", "std::refcell::RefCell", "std::sync::Mutex"]
the getter and setter will use the wrapped type, 
if the type doesn't derive Copy, it will panic beacause the default implement is directly deref the type.
