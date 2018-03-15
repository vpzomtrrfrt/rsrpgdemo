extern crate find_folder;
extern crate piston_window;
extern crate tiled;

use piston_window::{ImageSize, Transformed, RenderEvent, PressEvent, ReleaseEvent, UpdateEvent};

#[derive(Debug)]
struct Vec2 {
    x: f64,
    y: f64
}

fn main() {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let file = std::fs::File::open(assets.join("sewers.tmx")).unwrap();
    let map = tiled::parse(file).unwrap();

    let opengl = piston_window::OpenGL::V3_2;
    let mut window: piston_window::PistonWindow = piston_window::PistonWindow::new(
        opengl,
        0,
        piston_window::WindowSettings::new("rsrpgdemo", [640, 480])
        .exit_on_esc(true)
        .opengl(opengl)
        .srgb(false)
        .build()
        .unwrap()
        );

    let tileset = map.get_tileset_by_gid(1).unwrap();
    let tile_width = tileset.tile_width;
    let tile_height = tileset.tile_height;

    let max_width = f64::from(tile_width * 20);
    let max_height = f64::from(tile_height * 20);

    let tilesheet = assets.join(&tileset.images[0].source);
    let tilesheet = piston_window::Texture::from_path(
        &mut window.factory,
        &tilesheet,
        piston_window::Flip::None,
        &piston_window::TextureSettings::new()
        ).unwrap();
    let (sheet_width, _) = tilesheet.get_size();
    let layer = &map.layers[0];
    let image = piston_window::Image::new();

    let mut pos = Vec2 {
        x: 64.0,
        y: 64.0
    };
    let mut keyboard_state = std::collections::HashSet::new();

    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                piston_window::clear([0.1; 4], g);

                if let Some(viewport) = c.viewport {
                    let area = viewport.draw_size;

                    let scale = f64::from(area[0]) / max_width;

                    let trans = c.transform.trans(
                        f64::from(area[0]) / 2.0,
                        f64::from(area[1]) / 2.0)
                        .scale(scale, scale)
                        .trans(-pos.x, -pos.y);

                    for (y, row) in layer.tiles.iter().enumerate() {
                        for (x, &tile) in row.iter().enumerate() {
                            if tile == 0 {
                                continue;
                            }

                            let tile = tile - 1;

                            let src_rect = [
                                (tile % (sheet_width / tile_width) * tile_width) as f64,
                                (tile / (sheet_width / tile_height) * tile_height) as f64,
                                tile_width as f64,
                                tile_height as f64
                            ];

                            image.src_rect(src_rect).draw(
                                &tilesheet,
                                &Default::default(),
                                trans.trans(
                                    x as f64 * tile_width as f64,
                                    y as f64 * tile_height as f64
                                    ),
                                    g
                                    );
                        }
                    }
                }
            });
        }
        if let Some(args) = e.update_args() {
            let dir = Vec2 {
                x: if keyboard_state.contains(&piston_window::Key::Left) {-1.0} else {0.0}
                   + if keyboard_state.contains(&piston_window::Key::Right) {1.0} else {0.0},
                y: if keyboard_state.contains(&piston_window::Key::Down) {1.0} else {0.0}
                   + if keyboard_state.contains(&piston_window::Key::Up) {-1.0} else {0.0}
            };
            const SPEED: f64 = 128.0;
            pos.x += dir.x * args.dt * SPEED;
            pos.y += dir.y * args.dt * SPEED;
            println!("{:?}", pos);
        }
        if let Some(piston_window::Button::Keyboard(key)) = e.press_args() {
            keyboard_state.insert(key);
        }
        if let Some(piston_window::Button::Keyboard(key)) = e.release_args() {
            keyboard_state.remove(&key);
        }
    }
}
