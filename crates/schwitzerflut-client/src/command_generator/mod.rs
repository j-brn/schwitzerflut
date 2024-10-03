use schwitzerflut_protocol::command::Command;

mod image;
mod shard;
pub trait CommandGenerator {
    fn commands(&self) -> impl Iterator<Item = Command>;
}
