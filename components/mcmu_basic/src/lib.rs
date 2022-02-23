#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate anyhow;

pub mod profile;
pub mod protocol {
    pub mod mcmu;
    pub mod minecraft;
}
