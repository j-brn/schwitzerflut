use schwitzerflut_protocol::command::Command;
use schwitzerflut_protocol::coordinates::Coordinates;

mod image;

pub trait CommandGenerator {
    fn commands(&self) -> impl Iterator<Item = Command>;
}
