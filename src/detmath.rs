//! Transcendental functions routed through the pure-Rust `libm` crate.
//!
//! `f32::cos`, `f64::acos` and the like call the platform C library, whose results differ
//! in the last bit between glibc and the MSVC runtime. Every such call in this crate is
//! rewritten to the `d_`-prefixed method here, so results are identical on every
//! platform. `sqrt` is IEEE correctly rounded and stays native.
#![allow(dead_code)]

pub(crate) trait DetMath: Copy {
    fn d_sin(self) -> Self;
    fn d_cos(self) -> Self;
    fn d_tan(self) -> Self;
    fn d_asin(self) -> Self;
    fn d_acos(self) -> Self;
    fn d_atan(self) -> Self;
    fn d_atan2(self, other: Self) -> Self;
    fn d_sin_cos(self) -> (Self, Self);
    fn d_exp(self) -> Self;
    fn d_exp2(self) -> Self;
    fn d_exp_m1(self) -> Self;
    fn d_ln(self) -> Self;
    fn d_ln_1p(self) -> Self;
    fn d_log(self, base: Self) -> Self;
    fn d_log2(self) -> Self;
    fn d_log10(self) -> Self;
    fn d_powf(self, n: Self) -> Self;
    fn d_powi(self, n: i32) -> Self;
    fn d_cbrt(self) -> Self;
    fn d_hypot(self, other: Self) -> Self;
    fn d_mul_add(self, a: Self, b: Self) -> Self;
    fn d_tanh(self) -> Self;
    fn d_sinh(self) -> Self;
    fn d_cosh(self) -> Self;
    fn d_asinh(self) -> Self;
    fn d_acosh(self) -> Self;
    fn d_atanh(self) -> Self;
}

macro_rules! impl_det_math {
    ($t:ty, $sin:ident, $cos:ident, $tan:ident, $asin:ident, $acos:ident, $atan:ident,
     $atan2:ident, $sincos:ident, $exp:ident, $exp2:ident, $expm1:ident, $ln:ident,
     $ln1p:ident, $log2:ident, $log10:ident, $pow:ident, $cbrt:ident, $hypot:ident,
     $fma:ident, $tanh:ident, $sinh:ident, $cosh:ident, $asinh:ident, $acosh:ident,
     $atanh:ident) => {
        impl DetMath for $t {
            fn d_sin(self) -> Self { libm::$sin(self) }
            fn d_cos(self) -> Self { libm::$cos(self) }
            fn d_tan(self) -> Self { libm::$tan(self) }
            fn d_asin(self) -> Self { libm::$asin(self) }
            fn d_acos(self) -> Self { libm::$acos(self) }
            fn d_atan(self) -> Self { libm::$atan(self) }
            fn d_atan2(self, other: Self) -> Self { libm::$atan2(self, other) }
            fn d_sin_cos(self) -> (Self, Self) { libm::$sincos(self) }
            fn d_exp(self) -> Self { libm::$exp(self) }
            fn d_exp2(self) -> Self { libm::$exp2(self) }
            fn d_exp_m1(self) -> Self { libm::$expm1(self) }
            fn d_ln(self) -> Self { libm::$ln(self) }
            fn d_ln_1p(self) -> Self { libm::$ln1p(self) }
            fn d_log(self, base: Self) -> Self { libm::$ln(self) / libm::$ln(base) }
            fn d_log2(self) -> Self { libm::$log2(self) }
            fn d_log10(self) -> Self { libm::$log10(self) }
            fn d_powf(self, n: Self) -> Self { libm::$pow(self, n) }
            fn d_powi(self, n: i32) -> Self { libm::$pow(self, n as $t) }
            fn d_cbrt(self) -> Self { libm::$cbrt(self) }
            fn d_hypot(self, other: Self) -> Self { libm::$hypot(self, other) }
            fn d_mul_add(self, a: Self, b: Self) -> Self { libm::$fma(self, a, b) }
            fn d_tanh(self) -> Self { libm::$tanh(self) }
            fn d_sinh(self) -> Self { libm::$sinh(self) }
            fn d_cosh(self) -> Self { libm::$cosh(self) }
            fn d_asinh(self) -> Self { libm::$asinh(self) }
            fn d_acosh(self) -> Self { libm::$acosh(self) }
            fn d_atanh(self) -> Self { libm::$atanh(self) }
        }
    };
}

impl_det_math!(f32, sinf, cosf, tanf, asinf, acosf, atanf, atan2f, sincosf, expf, exp2f,
    expm1f, logf, log1pf, log2f, log10f, powf, cbrtf, hypotf, fmaf, tanhf, sinhf, coshf,
    asinhf, acoshf, atanhf);
impl_det_math!(f64, sin, cos, tan, asin, acos, atan, atan2, sincos, exp, exp2, expm1, log,
    log1p, log2, log10, pow, cbrt, hypot, fma, tanh, sinh, cosh, asinh, acosh, atanh);
