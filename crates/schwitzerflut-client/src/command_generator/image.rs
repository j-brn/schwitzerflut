use crate::command_generator::CommandGenerator;
use image::imageops::FilterType;
use image::{DynamicImage, ImageResult, RgbaImage};
use schwitzerflut_protocol::color::{Color, RgbColor, RgbaColor};
use schwitzerflut_protocol::command::{Command, SetPixelCommand};
use schwitzerflut_protocol::coordinates::Coordinates;
use std::iter;
use std::path::{Path, PathBuf};

/// Pixel source
pub struct Image {
    image: RgbaImage,
    offset: Coordinates,
    include_transparent_pixels: bool,
}

impl CommandGenerator for Image {
    fn commands(&self) -> impl Iterator<Item = Command> {
        self.image
            .enumerate_pixels()
            .filter(|(_, _, color)| self.include_transparent_pixels || color.0[3] != 0)
            .map(|(x, y, color)| {
                Command::SetPixel(SetPixelCommand {
                    coordinates: Coordinates {
                        x: x + self.offset.x,
                        y: y + self.offset.y,
                    },
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

pub struct CommandGeneratorBuilder {
    image: DynamicImage,
    offset: Option<Coordinates>,
    resize: Option<(u32, u32)>,
    include_transparent: bool,
}

impl CommandGeneratorBuilder {
    pub fn new(image: DynamicImage) -> Self {
        Self {
            image,
            offset: None,
            resize: None,
            include_transparent: false,
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> ImageResult<Self> {
        Ok(Self::new(image::open(path)?))
    }

    pub fn resize(mut self, size: (u32, u32)) -> Self {
        self.resize = Some(size);
        self
    }

    /// whether to generate commands for transparent pixels.
    ///
    ///
    pub fn include_transparent_pixels(mut self, include: bool) -> Self {
        self.include_transparent = include;
        self
    }

    pub fn offset(mut self, offset: Coordinates) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn build(self) -> Image {
        let mut image = self.image;

        if let Some((width, height)) = self.resize {
            image.resize(width, height, FilterType::CatmullRom);
        }

        Image {
            image: image.to_rgba8(),
            offset: self.offset.unwrap_or(Coordinates::new(0, 0)),
            include_transparent_pixels: self.include_transparent,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command_generator::image::{CommandGeneratorBuilder, Image};
    use crate::command_generator::CommandGenerator;
    use image::{DynamicImage, GenericImage, GenericImageView, Rgba, RgbaImage};
    use schwitzerflut_protocol::color::{Color, RgbColor, RgbaColor};
    use schwitzerflut_protocol::command::{Command, SetPixelCommand};
    use schwitzerflut_protocol::coordinates::Coordinates;

    fn get_test_image() -> DynamicImage {
        let mut img = DynamicImage::new_rgba8(4, 4);

        img.put_pixel(0, 0, Rgba([255, 0, 0, 1]));
        img.put_pixel(1, 0, Rgba([255, 0, 0, 1]));
        img.put_pixel(2, 0, Rgba([255, 0, 0, 1]));
        img.put_pixel(3, 0, Rgba([255, 0, 0, 1]));

        img.put_pixel(0, 1, Rgba([0, 255, 0, 1]));
        img.put_pixel(1, 1, Rgba([0, 255, 0, 1]));
        img.put_pixel(2, 1, Rgba([0, 255, 0, 1]));
        img.put_pixel(3, 1, Rgba([0, 255, 0, 1]));

        img.put_pixel(0, 2, Rgba([255, 255, 255, 0]));
        img.put_pixel(1, 2, Rgba([255, 255, 255, 0]));
        img.put_pixel(2, 2, Rgba([255, 255, 255, 0]));
        img.put_pixel(3, 2, Rgba([255, 255, 255, 0]));

        img.put_pixel(0, 3, Rgba([0, 0, 255, 1]));
        img.put_pixel(1, 3, Rgba([0, 0, 255, 1]));
        img.put_pixel(2, 3, Rgba([0, 0, 255, 1]));
        img.put_pixel(3, 3, Rgba([0, 0, 255, 1]));

        img
    }

    #[test]
    pub fn test_image() {
        let commands = CommandGeneratorBuilder::new(get_test_image())
            .include_transparent_pixels(true)
            .build()
            .commands()
            .collect::<Vec<_>>();

        let expected = vec![
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(0, 0),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 0, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(1, 0),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 0, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(2, 0),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 0, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(3, 0),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 0, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(0, 1),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 255, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(1, 1),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 255, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(2, 1),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 255, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(3, 1),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 255, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(0, 2),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 255, 255), 0)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(1, 2),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 255, 255), 0)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(2, 2),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 255, 255), 0)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(3, 2),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 255, 255), 0)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(0, 3),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 0, 255), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(1, 3),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 0, 255), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(2, 3),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 0, 255), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(3, 3),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 0, 255), 1)),
            )),
        ];

        assert_eq!(expected, commands);
    }

    #[test]
    pub fn test_image_without_transparent_pixels() {
        let commands = CommandGeneratorBuilder::new(get_test_image())
            .include_transparent_pixels(false)
            .build()
            .commands()
            .collect::<Vec<_>>();

        let expected = vec![
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(0, 0),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 0, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(1, 0),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 0, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(2, 0),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 0, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(3, 0),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 0, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(0, 1),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 255, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(1, 1),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 255, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(2, 1),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 255, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(3, 1),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 255, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(0, 3),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 0, 255), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(1, 3),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 0, 255), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(2, 3),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 0, 255), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(3, 3),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 0, 255), 1)),
            )),
        ];

        assert_eq!(expected, commands);
    }
    #[test]
    pub fn test_image_with_offset() {
        let commands = CommandGeneratorBuilder::new(get_test_image())
            .include_transparent_pixels(false)
            .offset(Coordinates::new(1, 1))
            .build()
            .commands()
            .collect::<Vec<_>>();

        let expected = vec![
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(1, 1),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 0, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(2, 1),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 0, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(3, 1),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 0, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(4, 1),
                Color::Rgba(RgbaColor::new(RgbColor::new(255, 0, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(1, 2),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 255, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(2, 2),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 255, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(3, 2),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 255, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(4, 2),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 255, 0), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(1, 4),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 0, 255), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(2, 4),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 0, 255), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(3, 4),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 0, 255), 1)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(4, 4),
                Color::Rgba(RgbaColor::new(RgbColor::new(0, 0, 255), 1)),
            )),
        ];

        assert_eq!(expected, commands);
    }
}
