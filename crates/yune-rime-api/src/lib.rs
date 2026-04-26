use std::os::raw::{c_char, c_int};

pub type RimeSessionId = usize;
pub type Bool = c_int;

pub const FALSE: Bool = 0;
pub const TRUE: Bool = 1;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RimeTraits {
    pub data_size: c_int,
    pub shared_data_dir: *const c_char,
    pub user_data_dir: *const c_char,
    pub distribution_name: *const c_char,
    pub distribution_code_name: *const c_char,
    pub distribution_version: *const c_char,
    pub app_name: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RimeCommit {
    pub data_size: c_int,
    pub text: *mut c_char,
}

#[must_use]
pub const fn bool_from(value: bool) -> Bool {
    if value {
        TRUE
    } else {
        FALSE
    }
}

#[cfg(test)]
mod tests {
    use super::{bool_from, FALSE, TRUE};

    #[test]
    fn maps_bool_to_rime_bool() {
        assert_eq!(bool_from(true), TRUE);
        assert_eq!(bool_from(false), FALSE);
    }
}
