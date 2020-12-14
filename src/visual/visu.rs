extern crate find_folder;
extern crate opengl_graphics;
extern crate piston_window;

use crate::board::utils::fdtos;
use crate::visual::text::{DrawText, TextAlignment, TextVerticalAlignment};
use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::*;

pub struct Visu {
    pub gl: GlGraphics,
    pub board: Vec<u16>,
    pub size: u16,
    pub time: String,
    pub margin_top: f64,
    pub margin_x: f64,
    pub number_scale: f64,
    pub index: i32,
    pub total_moves: i32,
    pub heuristic: String,
    pub explored_nodes: u32,
    pub max_path_len: u16
}

impl Visu {
    pub fn render(&mut self, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const LIGHT_GREY: [f32; 4] = [0.85, 0.85, 0.88, 1.0];
        const DARK_GREY: [f32; 4] = [0.18, 0.19, 0.19, 1.0];
        let size = self.size;
        let margin_top = self.margin_top;
        let margin_x = self.margin_x;
        let number_scale = self.number_scale;
        let win_w = args.window_size[0];
        let grid = grid::Grid {
            cols: size as u32,
            rows: size as u32,
            units: win_w / size as f64 - (margin_x * 2.0 / size as f64),
        };
        let line = Line::new(RED, 1.5);
        let board = self.board.clone();
        let time = self.time.clone();
        let heuristic = self.heuristic.clone();
        let total_moves = self.total_moves;
        let explored_nodes = self.explored_nodes;
        let max_path_len = self.max_path_len;
        let index = self.index;
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let ref font = assets.join("font.ttf");
        let mut glyph_cache = GlyphCache::new(font, (), TextureSettings::new()).unwrap();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(DARK_GREY, gl);
            grid.draw(
                &line,
                &c.draw_state,
                c.transform.trans(margin_x, margin_top),
                gl,
            );
            for y in 0..size as u32 {
                for x in 0..size as u32 {
                    let pos = grid.cell_position((x, y));
                    let nb = board[fdtos(x as u16, y as u16, size as u16) as usize];
                    if nb != size * size {
                        let string: String = nb.to_string();
                        let r = [
                            pos[0] + margin_x,
                            pos[1] + margin_top,
                            pos[0] + margin_x + grid.units,
                            pos[1] + margin_top + grid.units,
                        ];
                        gl.draw_text(
                            &string,
                            r,
                            LIGHT_GREY,
                            ((64.0 * (number_scale / size as f64)) as u32) as u32,
                            TextAlignment::Center,
                            TextVerticalAlignment::Center,
                            &mut glyph_cache,
                            &c,
                        );
                    }
                }
            }
            let mut r = [margin_x, 0.0, win_w - margin_x, margin_top];
            gl.draw_text(
                "NPUZZLE",
                r,
                LIGHT_GREY,
                64,
                TextAlignment::Center,
                TextVerticalAlignment::Center,
                &mut glyph_cache,
                &c,
            );
            let move_str = format!(
                "{} : {}/{}",
                "Move",
                index.to_string(),
                total_moves.to_string()
            );
            r = [
                0.0,
                win_w + margin_top,
                win_w,
                win_w + margin_top + 1.0 * 35.0,
            ];
            gl.draw_text(
                &move_str,
                r,
                LIGHT_GREY,
                32,
                TextAlignment::Center,
                TextVerticalAlignment::Center,
                &mut glyph_cache,
                &c,
            );

            let duration_str = format!("{} : {}s", "Duration", time);
            r = [
                0.0,
                win_w + margin_top + 1.0 * 35.0,
                win_w,
                win_w + margin_top + 2.0 * 35.0,
            ];
            gl.draw_text(
                &duration_str,
                r,
                LIGHT_GREY,
                32,
                TextAlignment::Center,
                TextVerticalAlignment::Center,
                &mut glyph_cache,
                &c,
            );
            let heuristic_str = format!("{} : {}", "Heuristic", heuristic);
            r = [
                0.0,
                win_w + margin_top + 2.0 * 35.0,
                win_w,
                win_w + margin_top + 3.0 * 35.0,
            ];
            gl.draw_text(
                &heuristic_str,
                r,
                LIGHT_GREY,
                32,
                TextAlignment::Center,
                TextVerticalAlignment::Center,
                &mut glyph_cache,
                &c,
            );
            let explored_nodes_str = format!(
                "{} : {}",
                "Complexity in time",
                explored_nodes.to_string(),
            );
            r = [
                0.0,
                win_w + margin_top + 3.0 * 35.0,
                win_w,
                win_w + margin_top + 4.0 * 35.0,
            ];
            gl.draw_text(
                &explored_nodes_str,
                r,
                LIGHT_GREY,
                32,
                TextAlignment::Center,
                TextVerticalAlignment::Center,
                &mut glyph_cache,
                &c,
            );
            let max_path_len_str = format!(
                "{} : {}",
                "Complexity in size",
                max_path_len.to_string(),
            );
            r = [
                0.0,
                win_w + margin_top + 4.0 * 35.0,
                win_w,
                win_w + margin_top + 5.0 * 35.0,
            ];
            gl.draw_text(
                &max_path_len_str,
                r,
                LIGHT_GREY,
                32,
                TextAlignment::Center,
                TextVerticalAlignment::Center,
                &mut glyph_cache,
                &c,
            );
            let moves_str = format!("{} : {}", "Number of moves", total_moves.to_string());
            r = [
                0.0,
                win_w + margin_top + 5.0 * 35.0,
                win_w,
                win_w + margin_top + 6.0 * 35.0,
            ];
            gl.draw_text(
                &moves_str,
                r,
                LIGHT_GREY,
                32,
                TextAlignment::Center,
                TextVerticalAlignment::Center,
                &mut glyph_cache,
                &c,
            );
        });
    }

    pub fn update_board(&mut self, _args: &Button, board: Vec<u16>, index: usize) {
        self.board = board;
        self.index = index as i32;
    }
}
