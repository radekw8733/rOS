#[cfg(target_arch = "x86_64")]
use x86_64::instructions::hlt;

pub fn halt() -> ! {
    #[cfg(target_arch = "x86_64" )]
    loop { hlt() }
}

pub fn detect_cpu_speed() {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::detect_cpu_speed();
}

pub fn prepare_userspace() {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::prepare_userspace();
}