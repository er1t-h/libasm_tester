#[repr(C)]
pub struct FtList {
    pub data: *mut cty::c_void,
    pub next: *mut FtList
}
