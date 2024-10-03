use crate::command_generator::CommandGenerator;
use schwitzerflut_protocol::command::Command;

/// CommandGenerator wrapper that implements simple modulus sharding
struct Shard<G>
where
    G: CommandGenerator,
{
    generator: G,
    shard: usize,
    num_shards: usize,
}

impl<G> CommandGenerator for Shard<G>
where
    G: CommandGenerator,
{
    fn commands(&self) -> impl Iterator<Item = Command> {
        self.generator
            .commands()
            .enumerate()
            .filter(|(index, _)| index % self.num_shards == self.shard)
            .map(|(_, command)| command)
    }
}

#[cfg(test)]
mod tests {
    use crate::command_generator::shard::Shard;
    use crate::command_generator::CommandGenerator;
    use schwitzerflut_protocol::color::{Color, RgbColor, RgbaColor};
    use schwitzerflut_protocol::command::{Command, SetPixelCommand};
    use schwitzerflut_protocol::coordinates::Coordinates;

    struct Generator(Vec<Command>);

    impl CommandGenerator for Generator {
        fn commands(&self) -> impl Iterator<Item = Command> {
            self.0.iter().cloned()
        }
    }

    #[test]
    fn test_shard() {
        let generator = Generator(vec![
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(1, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(2, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(3, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(4, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(5, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(6, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(7, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(8, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(9, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(10, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(11, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
        ]);

        let shard = Shard {
            generator,
            num_shards: 3,
            shard: 0,
        };

        let expected = vec![
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(1, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(4, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(7, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
            Command::SetPixel(SetPixelCommand::new(
                Coordinates::new(10, 1),
                Color::Rgb(RgbColor::new(255, 255, 255)),
            )),
        ];

        assert_eq!(expected, shard.commands().collect::<Vec<_>>());
    }
}
