/// RAII guard that flushes denormals to zero while active.
/// Wrap the entire `process()` call in this.
pub struct DenormalGuard {
    #[cfg(target_arch = "aarch64")]
    _previous_fpcr: u64,
    #[cfg(target_arch = "x86_64")]
    _previous_mxcsr: u32,
}

impl DenormalGuard {
    pub fn new() -> Self {
        #[cfg(target_arch = "aarch64")]
        {
            let fpcr: u64;
            unsafe {
                std::arch::asm!("mrs {}, fpcr", out(reg) fpcr);
                // Set FZ bit (bit 24) to flush denormals to zero
                let new_fpcr = fpcr | (1 << 24);
                std::arch::asm!("msr fpcr, {}", in(reg) new_fpcr);
            }
            Self { _previous_fpcr: fpcr }
        }

        #[cfg(target_arch = "x86_64")]
        {
            let mxcsr: u32;
            unsafe {
                mxcsr = std::arch::x86_64::_mm_getcsr();
                // Set DAZ (bit 6) and FTZ (bit 15)
                std::arch::x86_64::_mm_setcsr(mxcsr | (1 << 6) | (1 << 15));
            }
            Self { _previous_mxcsr: mxcsr }
        }

        #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
        Self {}
    }
}

impl Drop for DenormalGuard {
    fn drop(&mut self) {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            std::arch::asm!("msr fpcr, {}", in(reg) self._previous_fpcr);
        }

        #[cfg(target_arch = "x86_64")]
        unsafe {
            std::arch::x86_64::_mm_setcsr(self._previous_mxcsr);
        }
    }
}

/// Replace NaN/Inf with 0.0
#[inline]
pub fn sanitize(x: f32) -> f32 {
    if x.is_finite() { x } else { 0.0 }
}
