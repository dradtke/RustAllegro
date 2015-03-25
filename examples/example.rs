// This file is released into Public Domain.
#![feature(rustc_private)]
#![feature(collections)]
#![feature(core)]
#![feature(start)]
#![feature(path_ext)]

#[macro_use]
extern crate allegro;
extern crate allegro_image;
extern crate allegro_font;
extern crate allegro_ttf;
extern crate allegro_primitives;
extern crate getopts;

use getopts::*;
use std::env;
use std::fs::PathExt;
use std::path::Path;

use allegro::*;
use allegro_image::*;
use allegro_font::*;
use allegro_ttf::*;
use allegro_primitives::*;

allegro_main!
{
	let args = env::args().collect::<Vec<_>>();

    // If the working directory is the one where the executable was built, move up
    // a couple directories first so that paths resolve correctly. This makes the
    // example work even when double-clicked from a file browser.
    if let Some(exe) = Path::new(&args[0]).file_name() {
        if Path::new(exe).exists() {
            if let Err(e) = env::set_current_dir("../..") {
                panic!("{}", e);
            }
        }
    }

	let opts = vec![
		optflag("i", "init-only", "only initialize Allegro, don't do anything else")
	];

	let matches = getopts(args.tail(), opts.as_slice()).unwrap();

	let init_only = matches.opt_present("i");

	let mut core = Core::init().unwrap();
	ImageAddon::init(&core).unwrap();
	let font_addon = FontAddon::init(&core).unwrap();
	let ttf_addon = TtfAddon::init(&font_addon).unwrap();
	let prim = PrimitivesAddon::init(&core).unwrap();

	if init_only
	{
		return;
	}

	let disp = Display::new(&core, 800, 600).unwrap();
	disp.set_window_title("Rust example");

	core.install_keyboard().unwrap();
	core.install_mouse().unwrap();

	let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

	let q = EventQueue::new(&core).unwrap();
	q.register_event_source(&disp);
	q.register_event_source(&core.keyboard);
	q.register_event_source(&core.mouse);
	q.register_event_source(&timer);

	let bmp = Bitmap::new(&core, 256, 256).unwrap();

	let (mon_x1, mon_y1, mon_x2, mon_y2) = core.get_monitor_info(0).unwrap();
	println!("{} {} {} {}", mon_x1, mon_y1, mon_x2, mon_y2);

	core.set_target_bitmap(&bmp);
	core.clear_to_color(core.map_rgb_f(0.0, 0.0, 1.0));

	let sub_bmp = bmp.create_sub_bitmap(64, 64, 64, 64).unwrap();
	core.set_target_bitmap(&sub_bmp);
	core.clear_to_color(core.map_rgb_f(0.0, 1.0, 1.0));
	core.set_target_bitmap(disp.get_backbuffer());

	let bkg = Bitmap::load(&core, "data/mysha.pcx").unwrap();
	let font = Font::new_builtin(&font_addon).unwrap();
	let ttf = ttf_addon.load_ttf_font("data/DroidSans.ttf", -32, Flag::zero()).unwrap();
	let white = core.map_rgb_f(1.0, 1.0, 1.0);
	let black = core.map_rgb_f(0.0, 0.0, 0.0);

	let mut theta = 0.0f32;
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.clear_to_color(black);
			core.draw_bitmap(&bkg, 0.0, 0.0, Flag::zero());
			core.draw_rotated_bitmap(&bmp, 0.0, 0.0, (disp.get_width() / 2) as f32, (disp.get_height() / 2) as f32, theta, Flag::zero());
			core.draw_text(&font, white, (disp.get_width() / 2) as f32, 32.0, FontAlign::Centre, "Welcome to RustAllegro!");
			core.draw_text(&ttf, white, (disp.get_width() / 2) as f32, 96.0, FontAlign::Centre, "TTF text!");
			prim.draw_line(100.0, 200.0, 300.0, 200.0, white, 10.0);
			disp.flip();
			redraw = false;
		}

		match q.wait_for_event()
		{
			DisplayClose{source: src, ..} =>
			{
				assert!(disp.get_event_source().get_event_source() == src);
				println!("Display close event...");
				break 'exit;
			},
			KeyDown{keycode: k, ..} if k == KeyCode::Escape =>
			{
				println!("Pressed Escape!");
				break 'exit;
			},
			KeyChar{unichar: c, ..} =>
			{
				println!("Entered a character: {}", c);
			},
			TimerTick{..} =>
			{
				redraw = true;
				theta = theta + 0.01;
			},
			MouseButtonDown{button: b, ..} =>
			{
				println!("Mouse button {} pressed", b);
			},
			_ => println!("Some other event...")
		}
	}
}
