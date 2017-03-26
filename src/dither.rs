use std::ffi::{CStr, CString};
use errno::errno;
use libc::{self, c_void};

use caca::*;
use ::{CacaCanvas, Color, CacaError, CacaResult};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum DitherAntialias {
    None,
    Prefilter,
    Default,
    Unknown,
}

impl DitherAntialias {
    fn from_cstr(cs: &CStr) -> DitherAntialias {
        let string = cs.to_str().unwrap();
        match string {
            "none"      => DitherAntialias::None,
            "prefilter" => DitherAntialias::Prefilter,
            "default"   => DitherAntialias::Default,
            _           => DitherAntialias::Unknown,
        }
    }
    fn to_cstring(&self) -> CString {
        let antialias_name = match *self {
            DitherAntialias::Unknown   |
            DitherAntialias::None      => "none",
            DitherAntialias::Prefilter => "prefilter",
            DitherAntialias::Default   => "default",
        };
        CString::new(antialias_name).unwrap()
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum DitherColorMode {
    Mono,
    Gray,
    Ansi8,
    Ansi16,
    FullGray,
    Full8,
    Full16,
    Default,
    Unknown,
}

impl DitherColorMode {
    fn from_cstr(cs: &CStr) -> DitherColorMode {
        let string = cs.to_str().unwrap();
        match string {
            "mono"     => DitherColorMode::Mono,
            "gray"     => DitherColorMode::Gray,
            "8"        => DitherColorMode::Ansi8,
            "16"       => DitherColorMode::Ansi16,
            "fullgray" => DitherColorMode::FullGray,
            "full8"    => DitherColorMode::Full8,
            "full16"   => DitherColorMode::Full16,
            "default"  => DitherColorMode::Default,
            _          => DitherColorMode::Unknown,
        }
    }
    fn to_cstring(&self) -> CString {
        let color_mode_name = match *self {
            DitherColorMode::Unknown   |
            DitherColorMode::Default  => "default",
            DitherColorMode::Mono     => "mono",
            DitherColorMode::Gray     => "gray",
            DitherColorMode::Ansi8    => "8",
            DitherColorMode::Ansi16   => "16",
            DitherColorMode::FullGray => "fullgray",
            DitherColorMode::Full8    => "full8",
            DitherColorMode::Full16   => "full16",
        };
        CString::new(color_mode_name).unwrap()
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum DitherCharset {
    Ascii,
    Shades,
    Blocks,
    Default,
    Unknown
}

impl DitherCharset {
    fn from_cstr(cs: &CStr) -> DitherCharset {
        let string = cs.to_str().unwrap();
        println!("{}", string);
        match string {
            "ascii"   => DitherCharset::Ascii,
            "shades"  => DitherCharset::Shades,
            "blocks"  => DitherCharset::Blocks,
            "default" => DitherCharset::Default,
            _         => DitherCharset::Unknown,
        }
    }
    fn to_cstring(&self) -> CString {
        let charset_name = match *self {
            DitherCharset::Unknown   |
            DitherCharset::Default => "default",
            DitherCharset::Ascii   => "ascii",
            DitherCharset::Shades  => "shades",
            DitherCharset::Blocks  => "blocks",
        };
        CString::new(charset_name).unwrap()
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum DitherAlgorithm {
    None,
    Ordered2,
    Ordered4,
    Ordered8,
    Random,
    FloydSteinberg,
    Unknown,
}

impl DitherAlgorithm {
    fn from_cstr(cs: &CStr) -> DitherAlgorithm {
        let string = cs.to_str().unwrap();
        match string {
            "none"     => DitherAlgorithm::None,
            "ordered2" => DitherAlgorithm::Ordered2,
            "ordered4" => DitherAlgorithm::Ordered4,
            "ordered8" => DitherAlgorithm::Ordered8,
            "random"   => DitherAlgorithm::Random,
            "fstein"   => DitherAlgorithm::FloydSteinberg,
            _          => DitherAlgorithm::Unknown,
        }
    }
    fn to_cstring(&self) -> CString {
        let algorithm_name = match *self {
            DitherAlgorithm::Unknown   |
            DitherAlgorithm::FloydSteinberg => "fstein",
            DitherAlgorithm::None           =>"none",
            DitherAlgorithm::Ordered2       =>"ordered2",
            DitherAlgorithm::Ordered4       =>"ordered4",
            DitherAlgorithm::Ordered8       =>"ordered8",
            DitherAlgorithm::Random         =>"random",
        };
        CString::new(algorithm_name).unwrap()
    }
}

pub struct CacaDitherBuilder {
    bpp: i32,
    w: i32,
    h: i32,
    pitch: i32,
    mask: (u32, u32, u32, u32),
    palette: Option<[Color; 256]>,
    brightness: Option<f32>,
    gamma: Option<f32>,
    contrast: Option<f32>,
    antialias: Option<DitherAntialias>,
    color_mode: Option<DitherColorMode>,
    charset: Option<DitherCharset>,
    algorithm: Option<DitherAlgorithm>,
}

impl CacaDitherBuilder {
    pub fn build(&self) -> Result<CacaDither, CacaError>  {
        let dither = unsafe { caca_create_dither(self.bpp, self.w, self.h,
                                                 self.pitch,
                                                 self.mask.0, self.mask.1,
                                                 self.mask.2, self.mask.3)};

        match errno().0 {
            libc::EINVAL => Err(CacaError::InvalidDitherParams),
            libc::ENOMEM => Err(CacaError::NotEnoughMemory),
            _            => {
                let mut caca_dither = CacaDither { dither: dither };
                // if let Some(palette_) = self.palette {
                //     caca_dither.set_palette(palette_);
                // }
                if let Some(brightness_) = self.brightness {
                    caca_dither.set_brightness(brightness_)?;
                }
                if let Some(gamma_) = self.gamma {
                    caca_dither.set_gamma(gamma_)?;
                }
                if let Some(contrast_) = self.contrast {
                    caca_dither.set_contrast(contrast_)?;
                }
                if let Some(ref antialias_) = self.antialias {
                    caca_dither.set_antialias(antialias_);
                }
                if let Some(ref color_mode_) = self.color_mode {
                    caca_dither.set_color_mode(color_mode_);
                }
                if let Some(ref charset_) = self.charset {
                    caca_dither.set_charset(charset_);
                }
                if let Some(ref algorithm_) = self.algorithm {
                    caca_dither.set_algorithm(algorithm_);
                }
                Ok(caca_dither)
            }
        }
    }

    pub fn palette<'a>(&'a mut self, palette: [Color; 256]) -> &'a mut CacaDitherBuilder {
        self.palette = Some(palette);
        self
    }

    pub fn brightness<'a>(&'a mut self, brightness: f32) -> &'a mut CacaDitherBuilder {
        self.brightness = Some(brightness);
        self
    }

    pub fn gamma<'a>(&'a mut self, gamma: f32) -> &'a mut CacaDitherBuilder {
        self.gamma = Some(gamma);
        self
    }

    pub fn contrast<'a>(&'a mut self, contrast: f32) -> &'a mut CacaDitherBuilder {
        self.contrast = Some(contrast);
        self
    }

    pub fn antialias<'a>(&'a mut self, antialias: DitherAntialias) -> &'a mut CacaDitherBuilder {
        self.antialias = Some(antialias);
        self
    }

    pub fn color_mode<'a>(&'a mut self, color_mode: DitherColorMode) -> &'a mut CacaDitherBuilder {
        self.color_mode = Some(color_mode);
        self
    }

    pub fn charset<'a>(&'a mut self, charset: DitherCharset) -> &'a mut CacaDitherBuilder {
        self.charset = Some(charset);
        self
    }

    pub fn algorithm<'a>(&'a mut self, algorithm: DitherAlgorithm) -> &'a mut CacaDitherBuilder {
        self.algorithm = Some(algorithm);
        self
    }
}

pub struct CacaDither {
    dither: *mut CacaDitherRaw,
}

impl CacaDither {
    pub fn new(bpp: i32, w: i32, h: i32, pitch: i32,
               mask: (u32, u32, u32, u32)) -> CacaDitherBuilder {
        CacaDitherBuilder {
            bpp: bpp,
            w: w,
            h: h,
            pitch: pitch,
            mask: mask,
            palette: None,
            brightness: None,
            gamma: None,
            contrast: None,
            antialias: None,
            color_mode: None,
            charset: None,
            algorithm: None,
        }
    }

    #[cfg(never)]
    pub fn set_palette(&mut self, palette: &[Color; 256]) -> CacaResult {
        unsafe { caca_set_dither_palette(self.dither) };

        match errno().0 {
            libc::EINVAL => Err(CacaError::InvalidPalette),
            _            => Ok(())
        }
    }

    pub fn brightness(&self) -> f32 {
        unsafe { caca_get_dither_brightness(self.dither) }
    }

    pub fn set_brightness(&mut self, brightness: f32) -> CacaResult {
        unsafe { caca_set_dither_brightness(self.dither, brightness) };

        match errno().0 {
            libc::EINVAL => Err(CacaError::InvalidBrightness),
            _            => Ok(())
        }
    }

    pub fn gamma(&self) -> f32 {
        unsafe { caca_get_dither_gamma(self.dither) }
    }

    pub fn set_gamma(&mut self, gamma: f32) -> CacaResult {
        unsafe { caca_set_dither_gamma(self.dither, gamma) };

        match errno().0 {
            libc::EINVAL => Err(CacaError::InvalidGamma),
            _            => Ok(())
        }
    }

    pub fn contrast(&self) -> f32 {
        unsafe { caca_get_dither_contrast(self.dither) }
    }

    pub fn set_contrast(&mut self, contrast: f32) -> CacaResult {
        unsafe { caca_set_dither_contrast(self.dither, contrast) };

        match errno().0 {
            libc::EINVAL => Err(CacaError::InvalidContrast),
            _            => Ok(())
        }
    }

    pub fn antialias(&self) -> DitherAntialias {
        unsafe {
            let raw_str = caca_get_dither_antialias(self.dither);
            let antialias_name = CStr::from_ptr(raw_str);
            DitherAntialias::from_cstr(antialias_name)
        }
    }

    pub fn set_antialias(&mut self, antialias: &DitherAntialias) {
        // should never be invalid, due to the wrapped enum
        let antialias_cstring = antialias.to_cstring();
        unsafe { caca_set_dither_antialias(self.dither, antialias_cstring.as_ptr()) };
    }

    pub fn color_mode(&self) -> DitherColorMode {
        unsafe {
            let raw_str = caca_get_dither_color(self.dither);
            let color_name = CStr::from_ptr(raw_str);
            DitherColorMode::from_cstr(color_name)
        }
    }

    pub fn set_color_mode(&mut self, color_mode: &DitherColorMode) {
        // should never be invalid, due to the wrapped enum
        let color_mode_cstring = color_mode.to_cstring();
        unsafe { caca_set_dither_color(self.dither, color_mode_cstring.as_ptr()) };
    }

    pub fn charset(&self) -> DitherCharset {
        unsafe {
            let raw_str = caca_get_dither_charset(self.dither);
            let charset_name = CStr::from_ptr(raw_str);
            DitherCharset::from_cstr(charset_name)
        }
    }

    pub fn set_charset(&mut self, charset: &DitherCharset) {
        // should never be invalid, due to the wrapped enum
        let charset_cstring = charset.to_cstring();
        unsafe { caca_set_dither_charset(self.dither, charset_cstring.as_ptr()) };
    }

    pub fn algorithm(&self) -> DitherAlgorithm {
        unsafe {
            let raw_str = caca_get_dither_algorithm(self.dither);
            let algorithm_name = CStr::from_ptr(raw_str);
            DitherAlgorithm::from_cstr(algorithm_name)
        }
    }

    pub fn set_algorithm(&mut self, algorithm: &DitherAlgorithm) {
        // should never be invalid, due to the wrapped enum
        let algorithm_cstring = algorithm.to_cstring();
        unsafe { caca_set_dither_algorithm(self.dither, algorithm_cstring.as_ptr()) };
    }

    pub unsafe fn as_ptr(&self) -> *const CacaDitherRaw {
        self.dither
    }
}

impl<'a> CacaCanvas<'a> {
    pub fn dither_bitmap<T: Into<Vec<u8>>>(&mut self, x: i32, y: i32, w: i32, h: i32, dither: &CacaDither, image: T) {
        let image_buffer = image.into();
        unsafe { caca_dither_bitmap(self.canvas,
                                    x, y, w, h,
                                    dither.as_ptr(),
                                    image_buffer.as_ptr() as *const c_void) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let dither = CacaDither::new(8, 100, 100, 16, (0, 0, 0, 0))
        //.palette()
            .brightness(0.23)
            .gamma(0.75)
            .contrast(0.56)
            .antialias(DitherAntialias::Prefilter)
            .color_mode(DitherColorMode::Mono)
            .charset(DitherCharset::Blocks)
            .algorithm(DitherAlgorithm::Ordered4)
            .build().unwrap();
        assert_eq!(dither.brightness(), 0.23);
        assert_eq!(dither.gamma(), 0.75);
        assert_eq!(dither.contrast(), 0.56);
        assert_eq!(dither.antialias(), DitherAntialias::Prefilter);
        assert_eq!(dither.color_mode(), DitherColorMode::Mono);
        assert_eq!(dither.charset(), DitherCharset::Blocks);
        assert_eq!(dither.algorithm(), DitherAlgorithm::Ordered4);
    }
}
