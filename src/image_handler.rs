use arboard::{Clipboard, ImageData};
use std::path::PathBuf;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use image::{ColorType, ImageError, ImageFormat, ImageResult, RgbaImage};
use image::ColorType::Rgba8;
use image::error::{EncodingError, ImageFormatHint};
use image::ImageFormat::{Jpeg, Png, Gif};

#[derive(Clone)]
pub struct ImageHandler {
    pub buffer : Vec<u8>,
    width : u32,
    height : u32,
    color_type : ColorType
}

impl From<RgbaImage> for ImageHandler {
    fn from(value: RgbaImage) -> Self {
        Self{
            buffer: value.clone().into_raw(),
            width: value.width(),
            height: value.height(),
            color_type : Rgba8
        }
    }
}

impl ImageHandler {

    fn encode(handler : ImageHandler, path : String, format : ImageFormat) -> ImageResult<()> {
        let p = Path::new(&path);
        match format {
            Png => {
                println!("PNG encoding");
                let result = image::save_buffer_with_format(p,&handler.buffer,handler.width,handler.height,handler.color_type,Png);
                println!("PNG encoding end.");
                notifica::notify("PNG encoding end.", format!("PNG encoding end. File available: {}", path.as_str()).as_str())
                    .expect("OS API error.");
                result
            }
            Jpeg => {
                println!("JPEG encoding");
                let w = File::create(p)?;
                let w_buffer = BufWriter::with_capacity(handler.buffer.len(), w);
                let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(w_buffer, 85);
                let result = encoder.encode(&handler.buffer, handler.width, handler.height, handler.color_type);
                println!("JPEG encoding end.");
                notifica::notify("JPEG encoding end.", format!("JPEG encoding end. File available: {}", path.as_str()).as_str())
                    .expect("OS API error.");
                result
            }
            Gif => {
                println!("GIF encoding");
                let w = File::create(p)?;
                let w_buffer = BufWriter::with_capacity(handler.buffer.len(), w);
                let mut encoder = image::codecs::gif::GifEncoder::new_with_speed(w_buffer,10);
                let result = encoder.encode(&handler.buffer, handler.width, handler.height, handler.color_type);
                println!("GIF encoding end.");
                notifica::notify("GIF encoding end.", format!("GIF encoding end. File available: {}", path.as_str()).as_str())
                    .expect("OS API error.");
                result
            }
            _ => {
                let format_hint = ImageFormatHint::from(p);
                Err(ImageError::Encoding(EncodingError::from_format_hint(format_hint)))
            }
        }
    }

    pub fn to_clipboard(&self, cb: &mut Clipboard) -> Result<(), arboard::Error> {
        match notifica::notify("Screenshot saved in the clipboard.", "") {
            Ok(_) => {}
            Err(_) => {}
        }
        cb.set_image(ImageData {
            width: self.width as usize,
            height: self.height as usize,
            bytes: (&self.buffer).into(),
        })

    }

    pub fn save_image(&self, path: PathBuf) {
        let format : ImageFormat;
        match path.clone().extension(){
            Some(ext) => {
                match ext.to_str().unwrap() {
                    "png" => format = ImageFormat::Png,
                    "jpg" => format = ImageFormat::Jpeg,
                    "jpeg" => format = ImageFormat::Jpeg,
                    "gif" => format = ImageFormat::Gif,
                    _ => {
                        println!("Format not supported.");
                        return;
                    }
                }
            }
            None => {
                println!("Format not supported.");
                return;
            }
        }
        let _ = Self::encode(self.clone(),path.to_string_lossy().to_string(),format);
    }
}