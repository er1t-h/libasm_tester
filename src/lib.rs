#[repr(C)]
pub struct FtList {
    pub data: *mut cty::c_void,
    pub next: *mut FtList
}

impl Drop for FtList {
    fn drop(&mut self) {
        if !self.next.is_null() {
            drop(self.next);
            unsafe { libc::free(self as *mut FtList as *mut cty::c_void); }
        }
    }
}

extern "C" {
    // Mandatory
    pub fn ft_strlen(str: *const cty::c_char) -> cty::size_t;
    pub fn ft_strcpy(dest: *mut cty::c_char, src: *mut cty::c_char) -> *mut cty::c_char;
    pub fn ft_strcmp(s1: *const cty::c_char, s2: *const cty::c_char) -> cty::c_int;
    pub fn ft_write(fd: cty::c_int, buffer: *mut cty::c_void, count: cty::size_t) -> cty::ssize_t;
    pub fn ft_read(fd: cty::c_int, buffer: *mut cty::c_void, count: cty::size_t) -> cty::ssize_t;
    pub fn ft_strdup(str: *const cty::c_char) -> *mut cty::c_char;

    // Bonus
    pub fn ft_atoi_base(str: *const cty::c_char, base: *const cty::c_char) -> cty::c_int;
    pub fn ft_list_push_front(begin: *mut *const FtList, data: *const cty::c_void);
    pub fn ft_list_size(begin: *const FtList) -> cty::size_t;
}

#[cfg(test)]
mod tests;
