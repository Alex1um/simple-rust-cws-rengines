use std::rc::Rc;
use cws_rengines::geometry::position::Position;
use cws_rengines::objects::area::{Area};
use cws_rengines::objects::game_object::{GameObject, GameObjectRef};
use cws_rengines::renders::base::screen::Screen;
use cws_rengines::renders::base::view::View;
use cws_rengines::renders::sdl::render;
use cws_rengines::renders::sdl::render::{SDLRender};
use cws_rengines::events::event::Event;
use cws_rengines::events::event_loop::EventLoop;
use cws_rengines::renders::base::render::Render;
use std::env::*;

const AREA_MAX_X: usize = 17;
const AREA_MAX_Y: usize = 8;
const AREA_MAX_Z: usize = 3;

fn main() {
  let resolution = (800, 600);
  let mut area = Area::new(AREA_MAX_X, AREA_MAX_Y, AREA_MAX_Z).create_ref();
  let mut players: Vec<GameObjectRef> = Vec::new();
  let mut nishals: Vec<GameObjectRef> = Vec::new();
  let ply = GameObject::new(2, Position::new(AREA_MAX_X / 2, AREA_MAX_Y / 2, 2));
  {
    let mut area = area.borrow_mut();
    for x in 0..AREA_MAX_X {
      for y in 0..AREA_MAX_Y {
        let obj = GameObject::new(0, Position::new(x, y, 0));
        area.insert(obj).expect("Successful adding");
        if (y == 0 || y == AREA_MAX_Y - 1) && x % 2 == 0 || (x == 0 || x == AREA_MAX_X - 1) && y % 2 == 0 {
          let nish = GameObject::new(1, Position::new(x, y, 1));
          nishals.push(area.insert(nish).expect("Successful adding"));
        }
      }
    }
  }
  let ply = area.borrow_mut().insert(ply).expect("Successful adding player");
  players.push(ply);
  let view = View::new(&area, Position::new(0, 0, 0), AREA_MAX_X, AREA_MAX_Y, AREA_MAX_Z);
  let screen = Screen::new(view, resolution.0 / AREA_MAX_X, resolution.1 / AREA_MAX_Y);
  let (creator, mut render) = SDLRender::new(screen, resolution.0, resolution.1).expect("render created");
  let path = current_dir().expect("current dir");
  // render.load_textures(&creator, vec![
  //   path.join("assets/tile.png").to_str().expect("tile texture loaded"),
  //   path.join("assets/nishal.png").to_str().expect("nishal texture load"),
  //   path.join("assets/player.png").to_str().expect("player texture load"),
  //   path.join("assets/none.png").to_str().expect("none texture loads"),
  // ]);
  render.load_texture(&creator, path.join("assets/tile.png").to_str().expect("tile"));
  render.load_texture(&creator, path.join("assets/nishal.png").to_str().expect("nishal"));
  render.load_texture(&creator, path.join("assets/player.png").to_str().expect("player"));
  render.load_texture(&creator, path.join("assets/none.png").to_str().expect("none"));

  let mut mloop = EventLoop::new(Rc::clone(&area), render);

  let mut dx = 1isize;
  let mut dy = 1isize;
  mloop.add_event_listener(Event::Loop, Box::new(move ||
    {
      let mut new_pos = Position::new(0, 0, 0);
      {
        let Position { x: cx, y: cy, z: cz } = area.borrow().get_object_pos(ply).expect("play still in objects array");
        new_pos.set(cx, cy, cz);
        // println!("before: {} {}", cx, cy);
        if cx == AREA_MAX_X - 1 || cx == 0 {
          dx *= -1;
        }
        new_pos.x = ((new_pos.x as isize) + dx) as usize;
        if cy == AREA_MAX_Y - 1 || cy == 0 {
          dy *= -1;
        }
        new_pos.y = ((new_pos.y as isize) + dy) as usize;
        // println!("after: {} {}", new_pos.x, new_pos.y);
        area.borrow_mut().update_object(ply, new_pos);
        // player.set_pos(new_pos).expect("Successful setpos");
      }
    })).expect("Event listener added successfully");
  mloop.start();
}
