extern crate image;

use image::{GenericImage, ImageBuffer};

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
  let mut rom = File::open("roms/1.gb")?;
  let mut buffer = Vec::new();
  rom.read_to_end(&mut buffer)?;
  let rows: Vec<_> = buffer
    .chunks(2)
    .map(|row| {
      [
        ((row[0] >> 7) & 0x1) | (((row[1] >> 7) & 0x1) << 1),
        ((row[0] >> 6) & 0x1) | (((row[1] >> 6) & 0x1) << 1),
        ((row[0] >> 5) & 0x1) | (((row[1] >> 5) & 0x1) << 1),
        ((row[0] >> 4) & 0x1) | (((row[1] >> 4) & 0x1) << 1),
        ((row[0] >> 3) & 0x1) | (((row[1] >> 3) & 0x1) << 1),
        ((row[0] >> 2) & 0x1) | (((row[1] >> 2) & 0x1) << 1),
        ((row[0] >> 1) & 0x1) | (((row[1] >> 1) & 0x1) << 1),
        ((row[0] >> 0) & 0x1) | (((row[1] >> 0) & 0x1) << 1),
      ]
    })
    .collect();
  let tiles: Vec<_> = rows.chunks(8).collect();
  let tile_count = 8;
  let sections = tiles.chunks(tile_count * tile_count);
  let section_count = sections.len();
  let width = (8 * tile_count) as u32;
  let height = (section_count * 8 * tile_count) as u32;
  let mut image_map = ImageBuffer::new(width, height);
  for (section_i, section) in sections.enumerate() {
    let mut tile_i = 0;
    for tile in section {
      let tile_x = tile_i % tile_count;
      let tile_y = section_i * tile_count + tile_i / tile_count;
      for (y, row) in tile.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
          image_map.put_pixel(
            ((tile_x * 8) + x) as u32,
            ((tile_y * 8) + y) as u32,
            image::Luma([0xff - pixel * 0x4f]),
          );
        }
      }
      tile_i += 1;
    }
  }

  let mut i = 0;
  let mut y = 0;
  while y < height - width {
    let frame = image_map.sub_image(0, y, width, width).to_image();
    frame.save(format!("out/{}.png", i))?;
    i += 1;
    y = i * 8;
  }

  Ok(())
}
