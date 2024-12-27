#![allow(unused)]

use crate::command_generator::image::ImageSourceBuilder;
use crate::command_generator::shard::Shard;
use crate::command_generator::CommandGenerator;
use crate::stream::StreamWrapper;
use anyhow::Context;
use clap::Parser;
use image::DynamicImage;
use schwitzerflut_protocol::command::Command;
use schwitzerflut_protocol::coordinates::Coordinates;
use std::error::Error;
use std::fmt::format;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tokio::task;

mod command_generator;
mod stream;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(env)]
    address: SocketAddr,

    /// Path to the image to display
    #[arg(env)]
    image: PathBuf,

    #[arg(long, env, default_value_t = 0)]
    offset_x: u32,
    #[arg(long, env, default_value_t = 0)]
    offset_y: u32,

    #[arg(long, env, required = false)]
    height: Option<u32>,

    #[arg(long, env, required = false)]
    width: Option<u32>,

    /// Whether to send set pixel commands for transparent pixels
    #[arg(long, env, default_value_t = true)]
    skip_transparent_pixels: bool,

    /// Shards to handle with this client. If there is more than one connection configured,
    /// then shards are distributed across them
    #[arg(long, env, default_values_t = [1], value_delimiter=',')]
    shards: Vec<usize>,

    /// Total number of shards
    #[arg(long, env, default_value_t = 1)]
    num_shards: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let source = {
        let image = image::open(&args.image)
            .with_context(|| format!("unable to load image from {}", &args.image.display()))?;

        let mut builder = ImageSourceBuilder::new(image)
            .offset(Coordinates::new(args.offset_x, args.offset_y))
            .include_transparent_pixels(!args.skip_transparent_pixels);

        if let (Some(x), Some(y)) = (args.width, args.height) {
            builder = builder.resize((x, y));
        };

        builder.build()
    };

    let mut handles = Vec::new();

    for n in args.shards {
        let shard = Shard::new(source.clone(), n, args.num_shards);
        let payload = shard
            .commands()
            .map(|command| command.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        handles.push(tokio::task::spawn(async move {
            let stream = match StreamWrapper::new(args.address).connect().await {
                Ok(stream) => {
                    println!("shard {} connected successfully", n);
                    stream
                }
                Err(e) => {
                    eprintln!("shard {} failed to connect: {}", n, e);
                    return;
                }
            };

            loop {
                match stream.send(&payload).await {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                        break;
                    }
                }
            }

            println!("shard {} disconnected", n);
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
