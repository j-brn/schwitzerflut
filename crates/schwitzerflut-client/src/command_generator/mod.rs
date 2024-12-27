use schwitzerflut_protocol::command::Command;

pub mod image;
pub mod shard;
pub trait CommandGenerator {
    fn commands(&self) -> impl Iterator<Item = Command>;
}
