use x86_64::instructions::hlt;

pub fn halt() -> ! {
    #[cfg(feature = "x86_64" )]
    loop { hlt() }
}