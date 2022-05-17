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

// 0 - ocean
// 1 - river
// 2 - beach
// 3 - grassland
// 4 - forest
// 5 - mountain
// 6 - desert
// 7 - tundra
// 8 - snow
// 9 - swamp
const adj_matrix: [[f32; 10]; 10] = [
    [70.0, 10.0, 20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [10.0, 100.0, 5.0, 75.0, 50.0, 30.0, 5.0, 5.0, 5.0, 20.0],
    [100.0, 5.0, 150.0, 70.0, 20.0, 50.0, 70.0, 1.0, 1.0, 1.0],
    [1.0, 30.0, 10.0, 100.0, 80.0, 20.0, 5.0, 15.0, 1.0, 30.0],
    [1.0, 40.0, 1.0, 50.0, 100.0, 20.0, 1.0, 20.0, 1.0, 20.0],
    [1.0, 40.0, 1.0, 20.0, 30.0, 100.0, 10.0, 30.0, 50.0, 1.0],
    [1.0, 10.0, 0.0, 20.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0],
    [0.0, 5.0, 0.0, 15.0, 30.0, 30.0, 0.0, 100.0, 80.0, 60.0],
    [0.0, 15.0, 0.0, 0.0, 0.0, 80.0, 0.0, 0.0, 120.0, 0.0],
    [0.0, 30.0, 0.0, 30.0, 30.0, 0.0, 0.0, 50.0, 0.0, 100.0],
];

fn tileTypeToInt(tile: &TileType) -> usize {
    match tile {
        TileType::Ocean => 0,
        TileType::River => 1,
        TileType::Beach => 2,
        TileType::Grassland => 3,
        TileType::Forest => 4,
        TileType::Mountain => 5,
        TileType::Desert => 6,
        TileType::Tundra => 7,
        TileType::Snow => 8,
        TileType::Swamp => 9,
    }
}

fn add_prob_arrays(original: &mut [f32; 10], add: &[f32; 10]) {
    for i in 0..10 {
        original[i] += add[i];
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
    let ulType = ul.unwrap_or(TileType::Grassland);
    let upType = up.unwrap_or(TileType::Grassland);
    let urType = ur.unwrap_or(TileType::Grassland);
    let lType = l.unwrap_or(TileType::Grassland);
    let rType = r.unwrap_or(TileType::Grassland);
    let dlType = dl.unwrap_or(TileType::Grassland);
    let dType = d.unwrap_or(TileType::Grassland);
    let drType = dr.unwrap_or(TileType::Grassland);

    let mut weights = [0.0; 10];
    add_prob_arrays(&mut weights, &adj_matrix[tileTypeToInt(&ulType)]);
    add_prob_arrays(&mut weights, &adj_matrix[tileTypeToInt(&upType)]);
    add_prob_arrays(&mut weights, &adj_matrix[tileTypeToInt(&urType)]);
    add_prob_arrays(&mut weights, &adj_matrix[tileTypeToInt(&lType)]);
    add_prob_arrays(&mut weights, &adj_matrix[tileTypeToInt(&rType)]);
    add_prob_arrays(&mut weights, &adj_matrix[tileTypeToInt(&dlType)]);
    add_prob_arrays(&mut weights, &adj_matrix[tileTypeToInt(&dType)]);
    add_prob_arrays(&mut weights, &adj_matrix[tileTypeToInt(&drType)]);

    let sum_of_weigths = weights.iter().sum::<f32>();
    let mut rng = thread_rng();
    let rng_val = rng.gen_range(0.0..sum_of_weigths);
    let mut curr_sum = 0.0;
    let mut type_int = 0;

    for i in 0..10 {
        curr_sum += weights[i];
        if rng_val < curr_sum {
            type_int = i;
            break;
        }
    }

    return match type_int {
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
        _ => TileType::Grassland,
    };
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
