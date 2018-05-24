extern crate image;

use image::ImageBuffer;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
  let mut rom = File::open("roms/2.gb")?;
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
  let tile_count: u32 = 8;
  for (img_id, section) in tiles.chunks((tile_count * tile_count) as usize).enumerate() {
    let mut image = ImageBuffer::new(8 * tile_count, 8 * tile_count);
    let mut i = 0;
    for tile in section {
      let tile_x: u32 = i / tile_count;
      let tile_y: u32 = i % tile_count;
      for (y, row) in tile.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
          image.put_pixel(
            (tile_x * 8) + x as u32,
            (tile_y * 8) + y as u32,
            image::Luma([0xff - pixel * 0x4f]),
          );
        }
      }
      i += 1;
    }
    let path = format!("out/{}.png", img_id);
    image.save(path)?;
  }

  Ok(())
}
