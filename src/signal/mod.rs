extern crate libc;

use array::Array;
use defines::AfError;
use defines::InterpType;
use defines::ConvMode;
use defines::ConvDomain;
use self::libc::{uint8_t, c_int, c_float, c_double, c_longlong};

type MutAfArray = *mut self::libc::c_longlong;
type AfArray    = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_approx1(out: MutAfArray, inp: AfArray, pos: AfArray,
                  method: c_int, off_grid: c_float) -> c_int;

    fn af_approx2(out: MutAfArray, inp: AfArray, pos0: AfArray, pos1: AfArray,
                  method: c_int, off_grid: c_float) -> c_int;

    fn af_fft(out: MutAfArray, arr: AfArray,
              nfac: c_double, odim0: c_longlong) -> c_int;

    fn af_fft2(out: MutAfArray, arr: AfArray, nfac: c_double,
               odim0: c_longlong, odim1: c_longlong) -> c_int;

    fn af_fft3(out: MutAfArray, arr: AfArray, nfac: c_double,
               odim0: c_longlong, odim1: c_longlong, odim2: c_longlong) -> c_int;

    fn af_ifft(out: MutAfArray, arr: AfArray,
               nfac: c_double, odim0: c_longlong) -> c_int;

    fn af_ifft2(out: MutAfArray, arr: AfArray, nfac: c_double,
                odim0: c_longlong, odim1: c_longlong) -> c_int;

    fn af_ifft3(out: MutAfArray, arr: AfArray, nfac: c_double,
                odim0: c_longlong, odim1: c_longlong, odim2: c_longlong) -> c_int;

    fn af_fft_inplace(arr: AfArray, nfac: c_double) -> c_int;
    fn af_fft2_inplace(arr: AfArray, nfac: c_double) -> c_int;
    fn af_fft3_inplace(arr: AfArray, nfac: c_double) -> c_int;
    fn af_ifft_inplace(arr: AfArray, nfac: c_double) -> c_int;
    fn af_ifft2_inplace(arr: AfArray, nfac: c_double) -> c_int;
    fn af_ifft3_inplace(arr: AfArray, nfac: c_double) -> c_int;

    fn af_fft_r2c(out: MutAfArray, arr: AfArray,
                  nfac: c_double, pad0: c_longlong) -> c_int;
    fn af_fft2_r2c(out: MutAfArray, arr: AfArray,
                   nfac: c_double, pad0: c_longlong, pad1: c_longlong) -> c_int;
    fn af_fft3_r2c(out: MutAfArray, arr: AfArray,
                   nfac: c_double, pad0: c_longlong, pad1: c_longlong, pad2: c_longlong) -> c_int;

    fn af_fft_c2r(out: MutAfArray, input: AfArray, nfac: c_double, is_odd: c_int) -> c_int;
    fn af_fft2_c2r(out: MutAfArray, input: AfArray, nfac: c_double, is_odd: c_int) -> c_int;
    fn af_fft3_c2r(out: MutAfArray, input: AfArray, nfac: c_double, is_odd: c_int) -> c_int;

    fn af_convolve1(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t, d: uint8_t) -> c_int;
    fn af_convolve2(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t, d: uint8_t) -> c_int;
    fn af_convolve3(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t, d: uint8_t) -> c_int;
    fn af_convolve2_sep(o: MutAfArray, c: AfArray, r: AfArray, s: AfArray, m: uint8_t) -> c_int;
    fn af_fft_convolve1(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t) -> c_int;
    fn af_fft_convolve2(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t) -> c_int;
    fn af_fft_convolve3(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t) -> c_int;
    fn af_fir(out: MutAfArray, b: AfArray, x: AfArray) -> c_int;
    fn af_iir(out: MutAfArray, b: AfArray, a: AfArray, x: AfArray) -> c_int;
}

/// Perform signal interpolation for 1d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `pos` Array contains the interpolation locations
/// - `method` indicates the type of interpolation method that be used. It is of type enum
/// [InterpType](./enum.InterpType.html)
/// - `off_grid` is the value that will set in the output Array when certain index is out of bounds
///
/// # Return Values
///
/// An Array with interpolated values
#[allow(unused_mut)]
pub fn approx1(input: &Array, pos: &Array,
               method: InterpType, off_grid: f32) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_approx1(&mut temp as MutAfArray, input.get() as AfArray,
                                 pos.get() as AfArray, method as c_int, off_grid as c_float);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
/// Perform signal interpolation for 2d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `pos0` Array contains the interpolation locations for first dimension
/// - `pos1` Array contains the interpolation locations for second dimension
/// - `method` indicates the type of interpolation method that be used. It is of type enum
/// [InterpType](./enum.InterpType.html)
/// - `off_grid` is the value that will set in the output Array when certain index is out of bounds
///
/// # Return Values
///
/// An Array with interpolated values
pub fn approx2(input: &Array, pos0: &Array, pos1: &Array,
               method: InterpType, off_grid: f32) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_approx2(&mut temp as MutAfArray, input.get() as AfArray,
                                 pos0.get() as AfArray, pos1.get() as AfArray,
                                 method as c_int, off_grid as c_float);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Fast fourier transform for 1d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor with which the input is scaled before the
/// transformation is applied
/// - `odim0` is the length of output signals - used for either truncating or padding the input
/// signals
///
/// # Return Values
///
/// Transformed Array
#[allow(unused_mut)]
pub fn fft(input: &Array, norm_factor: f64, odim0: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fft(&mut temp as MutAfArray,
                             input.get() as AfArray, norm_factor as c_double,
                             odim0 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Fast fourier transform for 2d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor with which the input is scaled before the
/// transformation is applied
/// - `odim0` is the length of output signal first dimension - used for either truncating or padding the input
/// - `odim1` is the length of output signal second dimension - used for either truncating or padding the input
///
/// # Return Values
///
/// Transformed Array
#[allow(unused_mut)]
pub fn fft2(input: &Array, norm_factor: f64, odim0: i64, odim1: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fft2(&mut temp as MutAfArray,
                              input.get() as AfArray, norm_factor as c_double,
                              odim0 as c_longlong, odim1 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Fast fourier transform for 3d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor with which the input is scaled before the
/// transformation is applied
/// - `odim0` is the length of output signal first dimension - used for either truncating or padding the input
/// - `odim1` is the length of output signal second dimension - used for either truncating or padding the input
/// - `odim2` is the length of output signal third dimension - used for either truncating or padding the input
///
/// # Return Values
///
/// Transformed Array
#[allow(unused_mut)]
pub fn fft3(input: &Array, norm_factor: f64,
            odim0: i64, odim1: i64, odim2: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fft3(&mut temp as MutAfArray,
                              input.get() as AfArray, norm_factor as c_double,
                              odim0 as c_longlong, odim1 as c_longlong, odim2 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Inverse fast fourier transform for 1d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor with which the input is scaled before the
/// transformation is applied
/// - `odim0` is the length of output signals - used for either truncating or padding the input
/// signals
///
/// # Return Values
///
/// Transformed Array
#[allow(unused_mut)]
pub fn ifft(input: &Array, norm_factor: f64, odim0: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_ifft(&mut temp as MutAfArray,
                              input.get() as AfArray, norm_factor as c_double,
                              odim0 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Inverse fast fourier transform for 1d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor with which the input is scaled before the
/// transformation is applied
/// - `odim0` is the length of output signal first dimension - used for either truncating or padding the input
/// - `odim1` is the length of output signal second dimension - used for either truncating or padding the input
///
/// # Return Values
///
/// Transformed Array
#[allow(unused_mut)]
pub fn ifft2(input: &Array, norm_factor: f64, odim0: i64, odim1: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_ifft2(&mut temp as MutAfArray,
                               input.get() as AfArray, norm_factor as c_double,
                               odim0 as c_longlong, odim1 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Inverse fast fourier transform for 1d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor with which the input is scaled before the
/// transformation is applied
/// - `odim0` is the length of output signal first dimension - used for either truncating or padding the input
/// - `odim1` is the length of output signal second dimension - used for either truncating or padding the input
/// - `odim2` is the length of output signal third dimension - used for either truncating or padding the input
///
/// # Return Values
///
/// Transformed Array
#[allow(unused_mut)]
pub fn ifft3(input: &Array, norm_factor: f64,
             odim0: i64, odim1: i64, odim2: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_ifft3(&mut temp as MutAfArray,
                               input.get() as AfArray, norm_factor as c_double,
                               odim0 as c_longlong, odim1 as c_longlong, odim2 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

macro_rules! conv_func_def {
    ($fn_name:ident, $ffi_name: ident) => (
        /// Convolution
        ///
        /// The numeric suffix to the function name indicates the dimension in which the
        /// convolution operation is going to take place.
        ///
        /// - 1 - Indicates 1d convolution
        /// - 2 - Indicates 2d convolution
        /// - 3 - Indicates 3d convolution
        ///
        /// # Parameters
        ///
        /// - `signal` is the input signal
        /// - `filter` is the signal that shall be flipped for convolution operation
        /// - `mode` indicates if the convolution should be expanded or not(where output size
        /// equals input)
        /// - `domain` indicates if the convolution should be performed in frequencey or spatial
        /// domain
        ///
        /// # Return Values
        ///
        /// The convolved Array
        #[allow(unused_mut)]
        pub fn $fn_name(signal: &Array, filter: &Array,
                        mode: ConvMode, domain: ConvDomain) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray,
                                        signal.get() as AfArray, filter.get() as AfArray,
                                        mode as uint8_t, domain as uint8_t);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

conv_func_def!(convolve1, af_convolve1);
conv_func_def!(convolve2, af_convolve2);
conv_func_def!(convolve3, af_convolve3);

/// Separable convolution for 2d signals
///
/// # Parameters
///
/// - `cfilt` is the filter to be applied along coloumns
/// - `rfilt` is the filter to be applied along rows
/// - `signal` is the input signal
/// - `mode` indicates if the convolution should be expanded or not(where output size equals input)
///
/// # Return Values
///
/// The convolved Array
#[allow(unused_mut)]
pub fn convolve2_sep(cfilt: &Array, rfilt: &Array, signal: &Array,
                     mode: ConvMode) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_convolve2_sep(&mut temp as MutAfArray,
                                       cfilt.get() as AfArray, rfilt.get() as AfArray,
                                       signal.get() as AfArray, mode as uint8_t);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

macro_rules! fft_conv_func_def {
    ($fn_name:ident, $ffi_name: ident) => (
        /// Convolution using Fast-fourier transform
        ///
        /// The numeric suffix to the function name indicates the dimension in which the
        /// convolution operation is going to take place.
        ///
        /// - 1 - Indicates 1d convolution
        /// - 2 - Indicates 2d convolution
        /// - 3 - Indicates 3d convolution
        ///
        /// # Parameters
        ///
        /// - `signal` is the input signal
        /// - `filter` is the signal that shall be used for convolution operation
        /// - `mode` indicates if the convolution should be expanded or not(where output size
        /// equals input)
        ///
        /// # Return Values
        ///
        /// The convolved Array
        #[allow(unused_mut)]
        pub fn $fn_name(signal: &Array, filter: &Array,
                        mode: ConvMode) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray, signal.get() as AfArray,
                                        filter.get() as AfArray, mode as uint8_t);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

fft_conv_func_def!(fft_convolve1, af_fft_convolve1);
fft_conv_func_def!(fft_convolve2, af_fft_convolve2);
fft_conv_func_def!(fft_convolve3, af_fft_convolve3);

/// Finite impulse filter
///
/// # Parameters
///
/// - `b` is the Array containing the coefficients of the filter
/// - `x` is the input signal to filter
///
/// # Return Values
///
/// Filtered Array
#[allow(unused_mut)]
pub fn fir(b: &Array, x: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fir(&mut temp as MutAfArray, b.get() as AfArray, x.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Infinite impulse response filter
///
/// # Parameters
///
/// - `b` is the Array containing the feedforward coefficients
/// - `a` is the Array containing the feedback coefficients
/// - `x` is the input signal to filter
///
/// # Return Values
///
/// Filtered Array
#[allow(unused_mut)]
pub fn iir(b: &Array, a: &Array, x: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_iir(&mut temp as MutAfArray,
                             b.get() as AfArray, a.get() as AfArray, x.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// In place 1d dimensional Fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn fft_inplace(input: &Array, norm_factor: f64) -> Result<(), AfError> {
    unsafe {
        let err_val = af_fft_inplace(input.get() as AfArray, norm_factor as c_double);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// In place 2d dimensional Fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn fft2_inplace(input: &Array, norm_factor: f64) -> Result<(), AfError> {
    unsafe {
        let err_val = af_fft2_inplace(input.get() as AfArray, norm_factor as c_double);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// In place 3d dimensional Fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn fft3_inplace(input: &Array, norm_factor: f64) -> Result<(), AfError> {
    unsafe {
        let err_val = af_fft3_inplace(input.get() as AfArray, norm_factor as c_double);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// In place 1d dimensional inverse fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn ifft_inplace(input: &Array, norm_factor: f64) -> Result<(), AfError> {
    unsafe {
        let err_val = af_ifft_inplace(input.get() as AfArray, norm_factor as c_double);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// In place 2d dimensional inverse fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn ifft2_inplace(input: &Array, norm_factor: f64) -> Result<(), AfError> {
    unsafe {
        let err_val = af_ifft2_inplace(input.get() as AfArray, norm_factor as c_double);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// In place 3d dimensional inverse fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn ifft3_inplace(input: &Array, norm_factor: f64) -> Result<(), AfError> {
    unsafe {
        let err_val = af_ifft3_inplace(input.get() as AfArray, norm_factor as c_double);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// 1d Real to Complex fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor to be applied before fft is applied
/// - `pad0` is the padding along 0th dimension of Array
///
/// # Return Values
///
/// Complex Array
pub fn fft_r2c(input: &Array, norm_factor: f64, pad0: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fft_r2c(&mut temp as MutAfArray, input.get() as AfArray,
                                 norm_factor as c_double, pad0 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// 2d Real to Complex fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor to be applied before fft is applied
/// - `pad0` is the padding along 0th dimension of Array
/// - `pad1` is the padding along 1st dimension of Array
///
/// # Return Values
///
/// Complex Array
pub fn fft2_r2c(input: &Array, norm_factor: f64, pad0: i64, pad1: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fft2_r2c(&mut temp as MutAfArray, input.get() as AfArray,
                                  norm_factor as c_double, pad0 as c_longlong, pad1 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// 3d Real to Complex fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor to be applied before fft is applied
/// - `pad0` is the padding along 0th dimension of Array
/// - `pad1` is the padding along 1st dimension of Array
/// - `pad2` is the padding along 2nd dimension of Array
///
/// # Return Values
///
/// Complex Array
pub fn fft3_r2c(input: &Array, norm_factor: f64, pad0: i64, pad1: i64, pad2: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fft3_r2c(&mut temp as MutAfArray, input.get() as AfArray,
                                  norm_factor as c_double, pad0 as c_longlong,
                                  pad1 as c_longlong, pad2 as c_longlong);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// 1d Complex to Real fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor to be applied before fft is applied
/// - `is_odd` signifies if the output should be even or odd size
///
/// # Return Values
///
/// Complex Array
pub fn fft_c2r(input: &Array, norm_factor: f64, is_odd: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fft_c2r(&mut temp as MutAfArray, input.get() as AfArray,
                                 norm_factor as c_double, is_odd as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// 2d Complex to Real fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor to be applied before fft is applied
/// - `is_odd` signifies if the output should be even or odd size
///
/// # Return Values
///
/// Complex Array
pub fn fft2_c2r(input: &Array, norm_factor: f64, is_odd: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fft2_c2r(&mut temp as MutAfArray, input.get() as AfArray,
                                  norm_factor as c_double, is_odd as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// 3d Complex to Real fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor to be applied before fft is applied
/// - `is_odd` signifies if the output should be even or odd size
///
/// # Return Values
///
/// Complex Array
pub fn fft3_c2r(input: &Array, norm_factor: f64, is_odd: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fft3_c2r(&mut temp as MutAfArray, input.get() as AfArray,
                                  norm_factor as c_double, is_odd as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}
