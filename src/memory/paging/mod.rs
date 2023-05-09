use self::x86::X86Paging;

#[cfg(target_arch = "x86_64")]
pub mod x86;

pub trait PagingInterface {
    fn new() -> Self;
    fn allocate_pages_for_kernel(&mut self) -> Result<&'static str, &'static str>;
}

pub struct Paging {
    #[cfg(target_arch = "x86_64")]
    pub paging: X86Paging
}

impl PagingInterface for Paging {
    fn new() -> Self {
        #[cfg(target_arch = "x86_64")]
        Paging { paging: X86Paging::new() }
    }

    fn allocate_pages_for_kernel(&mut self) -> Result<&'static str, &'static str> {
        self.paging.allocate_pages_for_kernel()
    }
}