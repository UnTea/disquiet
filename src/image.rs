use crate::color::*;
use crate::error::*;
use crate::math::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::cell::UnsafeCell;
use std::alloc::handle_alloc_error;

pub struct RgbaImage {
    width: usize,
    height: usize,
    data: UnsafeCell<Vec<u8>>,
}

unsafe impl Sync for RgbaImage {}

impl RgbaImage {
    const NUM_COMPONENTS: usize = 4;

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: UnsafeCell::new(vec![0; Self::NUM_COMPONENTS * width * height]),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn pixel_offset(&self, x: usize, y: usize) -> usize {
        Self::NUM_COMPONENTS * (y * self.width + x)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Rgba {
        let offset = self.pixel_offset(x, y);

        let data = unsafe { &*self.data.get() };

        Rgba {
            r: data[offset + 0],
            g: data[offset + 1],
            b: data[offset + 2],
            a: data[offset + 3],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Rgba) {
        let offset = self.pixel_offset(x, y);

        let data = unsafe { &mut *self.data.get() };

        data[offset + 0] = color.r;
        data[offset + 1] = color.g;
        data[offset + 2] = color.b;
        data[offset + 3] = color.a;
    }

    /// Allows to change pixel color without requiring the image to be mutable
    ///
    /// This function is unsafe because it effectively enables mutable aliased
    /// memory. Callers must ensure that different threads never call this function
    /// on the same pixel.
    pub unsafe fn set_pixel_unsafe(&self, x: usize, y: usize, color: Rgba) {
        let offset = self.pixel_offset(x, y);

        let data = &mut *self.data.get();

        data[offset + 0] = color.r;
        data[offset + 1] = color.g;
        data[offset + 2] = color.b;
        data[offset + 3] = color.a;
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut buf_writer = BufWriter::new(file);

        let mut encoder = png::Encoder::new(&mut buf_writer, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(unsafe { &*self.data.get() })?;

        Ok(())
    }
}

pub struct Image {
    width: usize,
    height: usize,
    data: UnsafeCell<Vec<Float>>,
}

unsafe impl Sync for Image {}

impl Image {
    const NUM_COMPONENTS: usize = 3;

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: UnsafeCell::new(vec![0.0; Self::NUM_COMPONENTS * width * height]),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn pixel_offset(&self, x: usize, y: usize) -> usize {
        Self::NUM_COMPONENTS * (y * self.width + x)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color3 {
        let offset = self.pixel_offset(x, y);

        let data = unsafe { &*self.data.get() };

        Color3 {
            r: data[offset + 0],
            g: data[offset + 1],
            b: data[offset + 2],
        }
    }

    pub fn get_pixel_spherical(&self, phi: Float, theta: Float) -> Color3 {
        let w = self.width as Float;
        let h = self.height as Float;

        let u = 1.0 - (phi + PI) / (2.0 * PI);
        let v = (theta + PI/2.0) / PI;

        let x = modulo(w * u, w) as usize;
        let y = modulo(h * v, h) as usize;

        self.get_pixel(x, y)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color3) {
        let offset = self.pixel_offset(x, y);

        let data = unsafe { &mut *self.data.get() };

        data[offset + 0] = color.r;
        data[offset + 1] = color.g;
        data[offset + 2] = color.b;
    }

    /// Allows to change pixel color without requiring the image to be mutable
    ///
    /// This function is unsafe because it effectively enables mutable aliased
    /// memory. Callers must ensure that different threads never call this function
    /// on the same pixel.
    pub unsafe fn set_pixel_unsafe(&self, x: usize, y: usize, color: Color3) {
        let offset = self.pixel_offset(x, y);

        let data = &mut *self.data.get();

        data[offset + 0] = color.r;
        data[offset + 1] = color.g;
        data[offset + 2] = color.b;
    }
}
