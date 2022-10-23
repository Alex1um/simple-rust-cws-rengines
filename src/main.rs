use std::cell::RefCell;
use std::rc::Rc;
use cws_rengines::geometry::position::Position;
use cws_rengines::objects::area::{Area};
use cws_rengines::objects::game_object::{GameObject, GameObjectRef};
use cws_rengines::renders::base::screen::Screen;
use cws_rengines::renders::base::view::View;
use cws_rengines::renders::sdl::render;
use cws_rengines::renders::sdl::render::{Scene, SDLRender, Window};
use cws_rengines::events::event::Event;
use cws_rengines::events::event_loop::EventLoop;
use cws_rengines::renders::base::render::Render;
use std::env::*;

const AREA_MAX_X: usize = 17;
const AREA_MAX_Y: usize = 8;
const AREA_MAX_Z: usize = 3;

fn main() {
  let resolution = (800, 600);
  let mut area = Area::new(AREA_MAX_X, AREA_MAX_Y, AREA_MAX_Z);
  let view = View::new(Position::new(0, 0, 0), AREA_MAX_X, AREA_MAX_Y, AREA_MAX_Z);
  let screen = Screen::new(view, resolution.0 / AREA_MAX_X, resolution.1 / AREA_MAX_Y);
  let mut window = Window::new(resolution.0, resolution.1).expect("window created").create_ref();
  // let creator = Box::new(window.borrow().get_texture_creator());
  // let creator: &'static _ = Box::leak(creator);
  let creator = window.borrow().get_texture_creator();

  let mut scene = Scene::new(area);
  let mut players: Vec<GameObjectRef> = Vec::new();
  let mut nishals: Vec<GameObjectRef> = Vec::new();
  let ply = GameObject::new(2, Position::new(AREA_MAX_X / 2, AREA_MAX_Y / 2, 2));
  {
    for x in 0..AREA_MAX_X {
      for y in 0..AREA_MAX_Y {
        let obj = GameObject::new(0, Position::new(x, y, 0));
        scene.borrow_mut().add_object(obj).expect("Successful adding");
        if (y == 0 || y == AREA_MAX_Y - 1) && x % 2 == 0 || (x == 0 || x == AREA_MAX_X - 1) && y % 2 == 0 {
          let nish = GameObject::new(1, Position::new(x, y, 1));
          nishals.push(scene.borrow_mut().add_object(nish).expect("Successful adding"));
        }
      }
    }
  }
  let ply = scene.borrow_mut().add_object(ply).expect("Successful adding player");
  players.push(ply);
  let path = current_dir().expect("current dir");
  // render.load_textures(&creator, vec![
  //   path.join("assets/tile.png").to_str().expect("tile texture loaded"),
  //   path.join("assets/nishal.png").to_str().expect("nishal texture load"),
  //   path.join("assets/player.png").to_str().expect("player texture load"),
  //   path.join("assets/none.png").to_str().expect("none texture loads"),
  // ]);
  scene.borrow_mut().load_texture(creator, path.join("assets/tile.png").to_str().expect("tile"));
  scene.borrow_mut().load_texture(creator, path.join("assets/nishal.png").to_str().expect("nishal"));
  scene.borrow_mut().load_texture(creator, path.join("assets/player.png").to_str().expect("player"));
  scene.borrow_mut().load_texture(creator, path.join("assets/none.png").to_str().expect("none"));

  let render = SDLRender::new(screen, Rc::clone(&window));
  let mut mloop = EventLoop::new(Rc::clone(&scene), render, Rc::clone(&window));

  let mut dx = 1isize;
  let mut dy = 1isize;
  mloop.add_event_listener(Event::Loop, Box::new(move ||
    {
      let mut new_pos = Position::new(0, 0, 0);
      {
        let Position { x: cx, y: cy, z: cz } = scene.borrow().get_object_pos(ply).expect("play still in objects array");
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
        scene.borrow_mut().update_object(ply, new_pos).unwrap();
        // player.set_pos(new_pos).expect("Successful setpos");
      }
    })).expect("Event listener added successfully");
  mloop.start();
}
