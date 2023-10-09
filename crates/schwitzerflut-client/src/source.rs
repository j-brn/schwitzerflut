use image::imageops::FilterType;
use image::{ImageResult, RgbaImage};
use schwitzerflut_protocol::color::{Color, RgbColor, RgbaColor};
use schwitzerflut_protocol::command::{Command, SetPixelCommand};
use schwitzerflut_protocol::coordinates::Coordinates;
use std::path::{Path, PathBuf};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ImageRotation(u32);

impl ImageRotation {
    pub fn new(rotation: u32) -> Result<Self, &'static str> {
        match rotation {
            90 | 180 | 270 => Ok(Self(rotation)),
            _ => Err("rotation must be 90, 180 or 270"),
        }
    }
}

pub struct ImageSource {
    image: RgbaImage,
    skip_transparent_pixels: bool,
}

impl ImageSource {
    pub fn builder<P: AsRef<Path>>(path: P) -> ImageSourceBuilder<P> {
        ImageSourceBuilder::new(path)
    }

    pub fn as_commands(&self) -> impl Iterator<Item = Command> + '_ {
        self.image.enumerate_pixels().map(|(x, y, color)| {
            Command::SetPixel(SetPixelCommand {
                coordinates: Coordinates { x, y },
                color: Color::Rgba(RgbaColor {
                    rgb: RgbColor {
                        r: color[0],
                        g: color[1],
                        b: color[2],
                    },
                    alpha: color[3],
                }),
            })
        })
    }
}

pub struct ImageSourceBuilder<P: AsRef<Path>> {
    path: P,
    section: Option<(Coordinates, Coordinates)>,
    resize: Option<(u32, u32)>,
    skip_transparent: bool,
    flip_horizontally: bool,
    flip_vertically: bool,
    rotation: Option<ImageRotation>,
}

impl<P: AsRef<Path>> ImageSourceBuilder<P> {
    pub fn new(path: P) -> Self {
        Self {
            path,
            section: None,
            resize: None,
            skip_transparent: false,
            flip_vertically: false,
            flip_horizontally: false,
            rotation: None,
        }
    }

    pub fn resize(mut self, size: (u32, u32)) -> Self {
        self.resize = Some(size);
        self
    }

    pub fn crop(mut self, min: Coordinates, max: Coordinates) -> Self {
        self.section = Some((min, max));
        self
    }

    pub fn skip_transparent_pixels(mut self, skip: bool) -> Self {
        self.skip_transparent = skip;
        self
    }

    pub fn flip_horizontally(mut self) -> Self {
        self.flip_horizontally = true;
        self
    }

    pub fn flip_vertically(mut self) -> Self {
        self.flip_vertically = true;
        self
    }

    pub fn rotate(mut self, rotation: ImageRotation) -> Self {
        self.rotation = Some(rotation);
        self
    }

    pub fn build(&self) -> ImageResult<ImageSource> {
        let mut image = image::open(&self.path)?;

        if let Some((width, height)) = self.resize {
            image.resize(width, height, FilterType::Gaussian);
        }

        if let Some((start, dimensions)) = self.section {
            image.crop(start.x, start.y, dimensions.x, dimensions.y);
        }

        if self.flip_horizontally {
            image.fliph();
        }

        if self.flip_vertically {
            image.flipv();
        }

        if let Some(rotation) = self.rotation {
            match rotation {
                ImageRotation(90) => image.rotate90(),
                ImageRotation(180) => image.rotate180(),
                ImageRotation(270) => image.rotate270(),
                _ => unreachable!(),
            };
        }

        Ok(ImageSource {
            image: image.to_rgba8(),
            skip_transparent_pixels: self.skip_transparent,
        })
    }
}
