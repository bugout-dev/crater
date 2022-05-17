use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use rand::prelude::*;
pub struct Map {
    tiles: Vec<Vec<TileType>>,
    width: usize,
    height: usize,
}
#[derive(Debug, Clone, Copy)]
enum TileType {
    Ocean,
    River,
    Beach,
    Grassland,
    Forest,
    Mountain,
    Desert,
    Tundra,
    Snow,
    Swamp,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut tiles = Vec::new();
        for i in 0..height {
            let mut row: Vec<TileType> = Vec::new();
            for j in 0..width {
                let mut rng = thread_rng();
                let tile_type = rng.gen_range(0..10);
                let tile = match tile_type {
                    0 => TileType::Ocean,
                    1 => TileType::River,
                    2 => TileType::Beach,
                    3 => TileType::Grassland,
                    4 => TileType::Forest,
                    5 => TileType::Mountain,
                    6 => TileType::Desert,
                    7 => TileType::Tundra,
                    8 => TileType::Snow,
                    9 => TileType::Swamp,
                    10 => TileType::Ocean,
                    _ => TileType::Ocean,
                };
                row.push(tile);
            }
            tiles.push(row);
        }
        Map {
            tiles: tiles,
            width: width,
            height: height,
        }
    }

    pub fn scramble(&mut self) {
        let height = self.height;
        let width = self.width;
        for i in 0..height {
            for j in 0..width {
                let mut ul: Option<TileType> = None;
                let mut ur: Option<TileType> = None;
                let mut up: Option<TileType> = None;
                if i > 0 {
                    let up_row: &Vec<TileType> = &self.tiles[i - 1];
                    if j > 0 {
                        ul = Some(up_row[j - 1]);
                    }
                    if j < width - 1 {
                        ur = Some(up_row[j + 1]);
                    }

                    up = Some(up_row[j]);
                }
                let mut l: Option<TileType> = None;
                let mut r: Option<TileType> = None;
                if j > 0 {
                    l = Some(self.tiles[i][j - 1].clone());
                }
                if j < width - 1 {
                    r = Some(self.tiles[i][j + 1].clone());
                }
                let mut d: Option<TileType> = None;
                let mut dl: Option<TileType> = None;
                let mut dr: Option<TileType> = None;
                if i < height - 1 {
                    let down_row: &Vec<TileType> = &self.tiles[i + 1];
                    if j > 0 {
                        dl = Some(down_row[j - 1]);
                    }
                    if j < width - 1 {
                        dr = Some(down_row[j + 1]);
                    }
                    d = Some(down_row[j]);
                }
                self.tiles[i][j] = random_tile_type(ul, up, ur, l, r, dl, d, dr);
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &TileType {
        &self.tiles[y as usize][x as usize]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        self.tiles[y as usize][x as usize] = tile;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn render(&self, imageBuffer: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get_tile(x, y);
                let color = image::Rgb(tilePixel(&tile));
                imageBuffer.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

fn random_tile_type(
    ul: Option<TileType>,
    up: Option<TileType>,
    ur: Option<TileType>,
    l: Option<TileType>,
    r: Option<TileType>,
    dl: Option<TileType>,
    d: Option<TileType>,
    dr: Option<TileType>,
) -> TileType {
    let mut rng = thread_rng();
    let tile_type = rng.gen_range(0..74);

    let ulType = ul.unwrap_or(TileType::Grassland);
    let upType = up.unwrap_or(TileType::Grassland);
    let urType = ur.unwrap_or(TileType::Grassland);
    let lType = l.unwrap_or(TileType::Grassland);
    let rType = r.unwrap_or(TileType::Grassland);
    let dlType = dl.unwrap_or(TileType::Grassland);
    let dType = d.unwrap_or(TileType::Grassland);
    let drType = dr.unwrap_or(TileType::Grassland);

    match tile_type {
        0 => TileType::Ocean,
        1 => TileType::River,
        2 => TileType::Beach,
        3 => TileType::Grassland,
        4 => TileType::Forest,
        5 => TileType::Mountain,
        6 => TileType::Desert,
        7 => TileType::Tundra,
        8 => TileType::Snow,
        9 => TileType::Swamp,
        10..=17 => ulType,
        18..=25 => upType,
        26..=33 => urType,
        34..=41 => lType,
        42..=49 => rType,
        50..=57 => dlType,
        58..=65 => dType,
        66..=73 => drType,
        _ => TileType::Grassland,
    }
}

fn tilePixel(tileType: &TileType) -> [u8; 3] {
    match tileType {
        TileType::Ocean => [2, 45, 218],
        TileType::River => [11, 131, 191],
        TileType::Beach => [236, 219, 8],
        TileType::Grassland => [19, 236, 8],
        TileType::Forest => [20, 163, 10],
        TileType::Mountain => [64, 80, 74],
        TileType::Desert => [218, 183, 8],
        TileType::Tundra => [209, 234, 209],
        TileType::Snow => [255, 255, 255],
        TileType::Swamp => [77, 128, 99],
    }
}
