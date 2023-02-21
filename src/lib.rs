#[repr(C)]
pub struct FtList {
    pub data: *mut cty::c_void,
    pub next: *mut FtList
}

extern "C" {
    pub fn ft_strlen(str: *const cty::c_char) -> cty::size_t;
    pub fn ft_strcpy(dest: *mut cty::c_char, src: *mut cty::c_char) -> *mut cty::c_char;
    pub fn ft_strcmp(s1: *const cty::c_char, s2: *const cty::c_char) -> cty::c_int;
    pub fn ft_write(fd: cty::c_int, buffer: *mut cty::c_void, count: cty::size_t) -> cty::ssize_t;
    pub fn ft_read(fd: cty::c_int, buffer: *mut cty::c_void, count: cty::size_t) -> cty::ssize_t;
    pub fn ft_strdup(str: *const cty::c_char) -> *mut cty::c_char;
}

#[cfg(test)]
mod tests;
