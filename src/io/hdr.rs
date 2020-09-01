use byteorder::{ReadBytesExt, BigEndian};
use crate::error::*;
use crate::image::Image;
use crate::math::Float;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use crate::color::Color3;

/*

#?RADIANCE
# Made with Adobe Photoshop
GAMMA=1
PRIMARIES=0 0 0 0 0 0 0 0
FORMAT=32-bit_rle_rgbe

-Y 1024 +X 2048
 */

struct Header {
    width: usize,
    height: usize,
}

impl Header {
    fn parse<R: BufRead>(reader: &mut R) -> Result<Header> {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        if buf.trim() != "#?RADIANCE" {
            println!("hdr::parse(): invalid header:");
            return Err(Error::HdrDecodingError);
        }

        for line in reader.lines() {
            let line = line?;

            if line.is_empty() {
                break;
            }

            if line.starts_with("#") {
                continue;
            }

            let mut tokens = line.split("=");
            let parameter = tokens.next().ok_or(Error::HdrDecodingError)?;
            let value = tokens.next().ok_or(Error::HdrDecodingError)?;

            match parameter {
                "FORMAT" => if value != "32-bit_rle_rgbe" {
                    return Err(Error::HdrDecodingError);
                }
                "PRIMARIES" => (),
                "GAMMA" => (),
                _ => println!("unknown parameter: {}", parameter),
            }
        }

        buf.clear();
        reader.read_line(&mut buf)?;
        let mut tokens = buf.trim().split_whitespace();

        let y_axis = tokens.next().ok_or(Error::HdrDecodingError)?;
        let height = tokens.next().ok_or(Error::HdrDecodingError)?.parse()?;
        let x_axis = tokens.next().ok_or(Error::HdrDecodingError)?;
        let width = tokens.next().ok_or(Error::HdrDecodingError)?.parse()?;

        if y_axis != "-Y" || x_axis != "+X" {
            return Err(Error::HdrDecodingError);
        }

        Ok(Header {
            width,
            height
        })
    }
}

fn decode_rgbe(r: u8, g: u8, b: u8, e: u8) -> Color3 {
    let base: Float = 2.0;
    let diff = 128.0 + 8.0; // +8.0 scales RGB values from 0..255 to 0..1
    let exp = base.powf((e as Float) - diff);
    let r = (r as Float) * exp;
    let g = (g as Float) * exp;
    let b = (b as Float) * exp;

    Color3 { r, g, b }
}

fn unpack_rle_scanline<R: BufRead>(reader: &mut R, y: usize, image: &mut Image) -> Result<()> {
    let mut red = vec![0; image.width()];
    let mut green = vec![0; image.width()];
    let mut blue = vec![0; image.width()];
    let mut exp = vec![0; image.width()];

    let new_rle_indicator: u16 = reader.read_u16::<BigEndian>()?;
    if new_rle_indicator != 0x0202 {
        println!("hdr::parse(): only New RLE HDRs are supported");
        return Err(Error::HdrDecodingError);
    }

    let scanline_width: u16 = reader.read_u16::<BigEndian>()?;
    if scanline_width as usize != image.width() {
        println!("hdr::parse(): bad scanline width");
        return Err(Error::HdrDecodingError);
    }

    for component in &mut [&mut red, &mut green, &mut blue, &mut exp] {
        let mut x = 0;
        while x < image.width() {
            let count = reader.read_u8()?;
            if count > 128 {
                let count = count & 0x7F;
                let value = reader.read_u8()?;
                for _ in 0..count {
                    component[x] = value;
                    x += 1;
                }
            } else {
                for _ in 0..count {
                    component[x] = reader.read_u8()?;
                    x += 1;
                }
            }
        }
    }

    for x in 0..image.width() {
        let color = decode_rgbe(red[x], green[x], blue[x], exp[x]);
        image.set_pixel(x, y, color);
    }

    Ok(())
}

pub fn parse<P: AsRef<Path>>(p: P) -> Result<Image> {
    let mut reader = BufReader::new(File::open(p)?);
    let header = Header::parse(&mut reader)?;

    let mut image = Image::new(header.width, header.height);

    for y in (0..header.height).rev() {
        unpack_rle_scanline(&mut reader, y, &mut image)?;
    }

    Ok(image)
}