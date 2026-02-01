use libwebp_sys::*;
use std::ptr;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("LibWebP error")]
    WebPError,
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid configuration")]
    InvalidConfig,
    #[error("Memory allocation failed")]
    MemAllocFailed,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct EncoderOptions {
    pub minimize_size: bool,
    pub kmin: i32,
    pub kmax: i32,
    pub allow_mixed: bool,
    pub verbose: bool,
}

impl Default for EncoderOptions {
    fn default() -> Self {
        Self {
            minimize_size: false,
            kmin: 0,
            kmax: 0,
            allow_mixed: false,
            verbose: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FrameConfig {
    pub lossy: bool,
    pub quality: f32,
    pub method: i32,
    pub duration: i32,
    pub exact: bool, 
    pub near_lossless: i32,
    pub use_sharp_yuv: bool,
    pub alpha_quality: i32,
    pub alpha_filtering: i32,
    pub alpha_compression: i32,
}

impl Default for FrameConfig {
    fn default() -> Self {
        Self {
            lossy: false,
            quality: 75.0,
            method: 4,
            duration: 100,
            exact: false,
            near_lossless: 100,
            use_sharp_yuv: false,
            alpha_quality: 100,
            alpha_filtering: 1,
            alpha_compression: 1,
        }
    }
}

pub struct AnimationEncoder {
    enc: *mut WebPAnimEncoder,
    width: i32,
    height: i32,
    timestamp_ms: i32,
}

impl AnimationEncoder {
    pub fn new(width: i32, height: i32, options: &EncoderOptions) -> Result<Self> {
        let mut anim_config: WebPAnimEncoderOptions = unsafe { std::mem::zeroed() };
        if unsafe { WebPAnimEncoderOptionsInitInternal(&mut anim_config, WEBP_MUX_ABI_VERSION as i32) } == 0 {
            return Err(Error::WebPError);
        }

        anim_config.minimize_size = options.minimize_size as i32;
        anim_config.kmin = options.kmin;
        anim_config.kmax = options.kmax;
        anim_config.allow_mixed = options.allow_mixed as i32;
        anim_config.verbose = options.verbose as i32;

        let enc = unsafe { WebPAnimEncoderNewInternal(width, height, &anim_config, WEBP_MUX_ABI_VERSION as i32) };
        if enc.is_null() {
            return Err(Error::MemAllocFailed);
        }

        Ok(Self {
            enc,
            width,
            height,
            timestamp_ms: 0,
        })
    }

    pub fn add_frame(&mut self, img: &image::DynamicImage, config: &FrameConfig) -> Result<()> {
        if img.width() as i32 != self.width || img.height() as i32 != self.height {
             return Err(Error::InvalidConfig); 
        }

        let mut pic: WebPPicture = unsafe { std::mem::zeroed() };
        if unsafe { WebPPictureInitInternal(&mut pic, WEBP_ENCODER_ABI_VERSION as i32) } == 0 {
             return Err(Error::WebPError);
        }
        pic.use_argb = 1;
        pic.width = self.width;
        pic.height = self.height;
        
        let rgba = img.to_rgba8();
        let stride = self.width * 4; 
        
        if unsafe { WebPPictureImportRGBA(&mut pic, rgba.as_ptr(), stride) } == 0 {
             unsafe { WebPPictureFree(&mut pic) };
             return Err(Error::WebPError);
        }

        let mut webp_config: WebPConfig = unsafe { std::mem::zeroed() };
        if unsafe { WebPConfigInitInternal(&mut webp_config, WebPPreset::WEBP_PRESET_DEFAULT, 75.0, WEBP_ENCODER_ABI_VERSION as i32) } == 0 {
            unsafe { WebPPictureFree(&mut pic) };
            return Err(Error::WebPError);
        }

        webp_config.lossless = (!config.lossy) as i32;
        webp_config.quality = config.quality;
        webp_config.method = config.method;
        webp_config.exact = config.exact as i32;
        webp_config.near_lossless = config.near_lossless;
        webp_config.use_sharp_yuv = config.use_sharp_yuv as i32;
        webp_config.alpha_quality = config.alpha_quality;
        webp_config.alpha_filtering = config.alpha_filtering;
        webp_config.alpha_compression = config.alpha_compression;

        if unsafe { WebPValidateConfig(&webp_config) } == 0 {
             unsafe { WebPPictureFree(&mut pic) };
             return Err(Error::InvalidConfig);
        }

        if unsafe { WebPAnimEncoderAdd(self.enc, &mut pic, self.timestamp_ms, &webp_config) } == 0 {
             unsafe { WebPPictureFree(&mut pic) };
             return Err(Error::WebPError);
        }

        unsafe { WebPPictureFree(&mut pic) };
        
        self.timestamp_ms += config.duration;

        Ok(())
    }

    pub fn assemble(&mut self, loop_count: i32) -> Result<Vec<u8>> {
        if unsafe { WebPAnimEncoderAdd(self.enc, ptr::null_mut(), self.timestamp_ms, ptr::null()) } == 0 {
            return Err(Error::WebPError);
        }

        let mut webp_data: WebPData = unsafe { std::mem::zeroed() };

        if unsafe { WebPAnimEncoderAssemble(self.enc, &mut webp_data) } == 0 {
            return Err(Error::WebPError);
        }

        let mut final_data = unsafe { std::slice::from_raw_parts(webp_data.bytes, webp_data.size) }.to_vec();
        unsafe { WebPDataClear(&mut webp_data) };

        if loop_count > 0 {
             final_data = self.set_loop_count(&final_data, loop_count)?;
        }

        Ok(final_data)
    }
    
    fn set_loop_count(&self, data: &[u8], loop_count: i32) -> Result<Vec<u8>> {
         let mut webp_data: WebPData = unsafe { std::mem::zeroed() };
         webp_data.bytes = data.as_ptr();
         webp_data.size = data.len();
         
         let mux = unsafe { WebPMuxCreateInternal(&webp_data, 1, WEBP_MUX_ABI_VERSION as i32) };
         if mux.is_null() {
             return Err(Error::WebPError);
         }
         
         let mut new_params: WebPMuxAnimParams = unsafe { std::mem::zeroed() };
         let mut err = unsafe { WebPMuxGetAnimationParams(mux, &mut new_params) };
         if err != WebPMuxError::WEBP_MUX_OK {
              unsafe { WebPMuxDelete(mux) };
              return Err(Error::WebPError);
         }
         
         new_params.loop_count = loop_count;
         err = unsafe { WebPMuxSetAnimationParams(mux, &new_params) };
         if err != WebPMuxError::WEBP_MUX_OK {
              unsafe { WebPMuxDelete(mux) };
              return Err(Error::WebPError);
         }
         
         let mut output_data: WebPData = unsafe { std::mem::zeroed() };
         err = unsafe { WebPMuxAssemble(mux, &mut output_data) };
         unsafe { WebPMuxDelete(mux) };
         
         if err != WebPMuxError::WEBP_MUX_OK {
             return Err(Error::WebPError);
         }
         
         let res = unsafe { std::slice::from_raw_parts(output_data.bytes, output_data.size) }.to_vec();
         unsafe { WebPDataClear(&mut output_data) };
         
         Ok(res)
    }
}

impl Drop for AnimationEncoder {
    fn drop(&mut self) {
        if !self.enc.is_null() {
            unsafe { WebPAnimEncoderDelete(self.enc) };
        }
    }
}

pub fn read_image(path: &str) -> Result<image::DynamicImage> {
    Ok(image::open(path)?)
}