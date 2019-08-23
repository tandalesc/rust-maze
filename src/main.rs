extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::{thread, time};

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 0.8, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const RESOLUTION: [u32; 2] = [1280, 800];

const SQUARE_SIZE: f64 = 30.0;
const MEMORY_STRENGTH: f64 = 1.5;
const POSSIBLE_MOVES: [[i32; 2]; 4] = [
    [-1,0],
    [1,0],
    [0,1],
    [0,-1],
    //diagonal moves
    //[1,1],
    //[-1,1],
    //[-1,-1],
    //[1,-1]
];

pub struct Maze {
    maze_layout: Vec<Vec<i32>>,
    maze_size_x: usize,
    maze_size_y: usize,
    goal_x: usize,
    goal_y: usize
}

struct Player {
    position_x: usize,
    position_y: usize,
    maze_memory: Vec<Vec<f64>>
}

pub struct App {
    gl: GlGraphics, //OpenGL backend
    maze: Maze,
    player: Player
}

impl App {
    fn new(gl: GlGraphics) -> Self {
        App {
            gl: gl,
            maze: Maze {
                maze_size_x: 16,
                maze_size_y: 16,
                goal_x: 7,
                goal_y: 7,
                maze_layout: vec![
                    vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                    vec![0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
                    vec![0,1,0,0,0,0,0,0,0,0,0,0,0,0,1,0],
                    vec![0,1,0,1,1,1,1,1,1,1,1,1,1,0,1,0],
                    vec![0,1,0,1,0,0,0,0,0,0,0,0,1,0,1,0],
                    vec![0,1,0,1,0,1,1,0,1,1,1,0,1,0,1,0],
                    vec![0,1,0,1,0,1,0,0,0,0,1,0,1,0,1,0],
                    vec![0,1,0,1,0,1,0,0,0,0,1,0,1,0,1,0],
                    vec![0,1,0,1,0,1,0,0,0,0,1,0,1,0,1,0],
                    vec![0,1,0,1,0,1,0,0,0,0,1,0,1,0,1,0],
                    vec![0,1,0,1,0,1,1,1,1,1,1,0,0,0,1,0],
                    vec![0,1,0,1,0,0,0,0,0,0,0,0,1,0,1,0],
                    vec![0,1,0,1,1,1,1,1,1,1,1,1,1,0,1,0],
                    vec![0,1,0,0,0,0,0,0,0,0,0,0,0,0,1,0],
                    vec![0,1,1,1,1,1,1,1,1,1,1,1,1,0,1,0],
                    vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                ]
            },
            player: Player {
                position_x: 0,
                position_y: 0,
                maze_memory: vec![vec![0.0; 16]; 16]
            }
        }
    }
    fn initialize(&mut self) {
        for r in 0..self.maze.maze_size_y {
            for c in 0..self.maze.maze_size_x {
                self.player.maze_memory[r][c] = self.objective(c, r);
            }
        }
    }
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let square = rectangle::square(0.0, 0.0, SQUARE_SIZE);
        let maze_size_x = self.maze.maze_size_x as f64;
        let current_layout = self.maze.maze_layout.clone();
        let player_memory = self.player.maze_memory.clone();
        let ptx = (self.player.position_x as f64)*SQUARE_SIZE;
        let pty = (self.player.position_y as f64)*SQUARE_SIZE;
        let gx = (self.maze.goal_x as f64)*SQUARE_SIZE;
        let gy = (self.maze.goal_y as f64)*SQUARE_SIZE;

        self.gl.draw(args.viewport(), |c, gl| {
            clear([1.0; 4], gl);
            //draw maze
            for (row_i, row) in current_layout.iter().enumerate() {
                //println!("{}",row.to_string());
                for (col_i, col) in row.iter().enumerate() {
                    let tx = (col_i as f64)*SQUARE_SIZE;
                    let ty = (row_i as f64)*SQUARE_SIZE;
                    if *col==1 {
                        rectangle(BLUE, square, c.transform.trans(tx, ty), gl);
                    } else {
                        rectangle([0.0,0.0,1.0,0.1], square, c.transform.trans(tx, ty), gl);
                    }
                }
            }
            //draw memory map
            for (row_i, row) in player_memory.iter().enumerate() {
                for (col_i, col) in row.iter().enumerate() {
                    if *col!=0.0 {
                        let tx = (col_i as f64)*SQUARE_SIZE + (maze_size_x*SQUARE_SIZE);
                        let ty = (row_i as f64)*SQUARE_SIZE;
                        let opacity: f32 = ((*col as f32) / 25.0).min(1.0);
                        rectangle([0.0, 1.0, 0.0, opacity], square, c.transform.trans(tx, ty), gl);
                    }
                }
            }
            //draw player and goal
            rectangle(YELLOW, square, c.transform.trans(gx, gy), gl);
            rectangle(RED, square, c.transform.trans(ptx, pty), gl);
        });
        //artifically slow down simulation
        thread::sleep(time::Duration::from_millis(50));
    }

    fn objective(&self, new_x: usize, new_y: usize) -> f64 {
        if self.maze.maze_layout[new_y][new_x]!=0 {
            std::f64::MAX
        } else {
            let gx = self.maze.goal_x as f64;
            let gy = self.maze.goal_y as f64;
            let mx = new_x as f64;
            let my = new_y as f64;
            //distance formula
            ((mx-gx)*(mx-gx)+(my-gy)*(my-gy)).sqrt()
        }
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.player.position_x == self.maze.goal_x && self.player.position_y == self.maze.goal_y {
            return;
        }

        let ptx = self.player.position_x as f64;
        let pty = self.player.position_y as f64;
        let maze_size_x = self.maze.maze_size_x as f64;
        let maze_size_y = self.maze.maze_size_y as f64;
        let current_layout = self.maze.maze_layout.clone();
        //filter out disallowed moves
        let mut allowed_moves: Vec<_> = POSSIBLE_MOVES.iter().filter(|m| {
            let mtx = ptx + (m[0] as f64);
            let mty = pty + (m[1] as f64);
            if mtx < 0.0 || mty < 0.0 || mtx >= maze_size_x || mty >= maze_size_y {
                false
            } else {
                if current_layout[mty as usize][mtx as usize] == 0 {
                    true
                } else {
                    false
                }
            }
        }).collect();
        //sort by cost function per position
        allowed_moves.sort_by(|a, b| {
            let ax = ptx + (a[0] as f64);
            let ay = pty + (a[1] as f64);
            let aobj = self.player.maze_memory[ay as usize][ax as usize];
            let bx = ptx + (b[0] as f64);
            let by = pty + (b[1] as f64);
            let bobj = self.player.maze_memory[by as usize][bx as usize];
            aobj.partial_cmp(&bobj).unwrap()
        });
        //pull the first move (best move)
        if let Some(m) = allowed_moves.get(0) {
            let new_x = (self.player.position_x as i32) + m[0];
            let new_y = (self.player.position_y as i32) + m[1];
            self.player.maze_memory
                [self.player.position_y]
                [self.player.position_x] += MEMORY_STRENGTH;
            self.player.position_x = new_x as usize;
            self.player.position_y = new_y as usize;
        } else {
            println!("no moves allowed");
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Maze", RESOLUTION)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    //holds state information of our application
    let mut app = App::new(GlGraphics::new(opengl));
    app.initialize();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }

}
