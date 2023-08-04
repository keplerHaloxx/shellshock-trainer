///
/// A simple (non intrusive) trainer for http://www.shellshocklive.com/
///

mod platform;
mod math;

extern crate core;
use core::fmt;
use platform::{Cursor, Handle, VK};

use std::{thread};
use std::time;
use std::collections::BTreeMap;
use std::fmt::Formatter;

const SHOW_MAX_HITS: usize = 5;

#[derive(Debug,PartialEq,PartialOrd)]
enum Mode {
    ANGLE,
    VELOCITY,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Mode::ANGLE => write!(f, "ANGLE"),
            Mode::VELOCITY => write!(f, "VELOCITY")
        }
    }
}

fn main() {
    println!("[INFO] Searching ...");
    let handle = if cfg!(target_os = "windows") {
        platform::windows::find_shellshock_handle()
    } else {
        unimplemented!() // TODO implement linux, macos
    };

    println!("[INFO] ShellShock found. Waiting for input ...");
    start_event_loop(handle);
}

fn start_event_loop<H: Handle>(handle: H) {
    let mut mode = Mode::VELOCITY;
    let mut source = None;
    let mut target = None;

    let mut vk1_state = false;
    let mut vk2_state = false;
    let mut vk3_state = false;
    let mut vk4_state = false;
    let mut vk5_state = false;
    let mut vk6_state = false;

    println!();
    print_info(&None, &None, &mode);

    loop {
        thread::sleep(time::Duration::from_millis(10)); // sleep duration in milliseconds

        let vk1_key_down = handle.is_key_pressed(VK::Key1);
        let vk2_key_down = handle.is_key_pressed(VK::Key2);
        let vk3_key_down = handle.is_key_pressed(VK::Key3);
        let vk4_key_down = handle.is_key_pressed(VK::Key4);
        let vk5_key_down = handle.is_key_pressed(VK::Key5);
        let vk6_key_down = handle.is_key_pressed(VK::Key6);

        // Set position 1
        if vk1_key_down && !vk1_state {
            vk1_state = true;

            let position = handle.get_mouse_position_in_window();
            source = Some(position);

            clearscreen::clear().unwrap();
            print_info(&source, &target, &mode);
        } else if !vk1_key_down {
            vk1_state = false
        }

        // Set position 2
        if vk2_key_down && !vk2_state {
            vk2_state = true;

            let position = handle.get_mouse_position_in_window();
            target = Some(position);

            clearscreen::clear().unwrap();
            print_info(&source, &target, &mode);
        } else if !vk2_key_down {
            vk2_state = false
        }

        // Calculate hits
        if vk3_key_down && !vk3_state {
            vk3_state = true;

            if source.is_some() && target.is_some() {
                let rect = handle.get_window_rect();
                let from = source.as_ref().unwrap();
                let to = target.as_ref().unwrap();

                let target_pos = math::translate_target_position_relative_to_origin(&rect, from, to);

                let hits = match mode {
                    Mode::ANGLE => math::calc_launch_angles(target_pos.0, target_pos.1),
                    Mode::VELOCITY => math::calc_launch_velocities(target_pos.0, target_pos.1),
                };

                print_hits(hits);
            }
        } else if !vk3_key_down {
            vk3_state = false
        }

        // Clear positions
        if vk4_key_down && !vk4_state {
            vk4_state = true;

            source = None;
            target = None;

            clearscreen::clear().unwrap();
            print_info(&source, &target, &mode);
        } else if !vk4_key_down {
            vk4_state = false
        }

        // Switch calculation mode
        if vk5_key_down && !vk5_state {
            vk5_state = true;

            mode = if mode == Mode::ANGLE {
                Mode::VELOCITY
            } else {
                Mode::ANGLE
            };

            clearscreen::clear().unwrap();
            print_info(&source, &target, &mode);
        } else if !vk5_key_down {
            vk5_state = false
        }

        // Clear console
        if vk6_key_down && !vk6_state {
            vk6_state = true;

            clearscreen::clear().expect("Failed to clear screen");

            clearscreen::clear().unwrap();
            print_info(&source, &target, &mode)
        } else if !vk6_key_down {
            vk6_state = false
        }
    }
}

fn print_hits(hits: Vec<math::Hit>) {
    println!("[INFO] Results:");

    println!("Best -> {}",
             format_hits(&hits.iter().map(|hit| hit).collect::<Vec<_>>()));

    let categories = into_angle_categories(&hits);
    for (category, category_hits) in &categories {
        println!("{} -> {}", category, format_hits(&category_hits));
    }
}

fn format_hits(hits: &[&math::Hit]) -> String {
    hits.iter()
        .take(SHOW_MAX_HITS)
        .map(|hit| format!("{}", hit))
        .collect::<Vec<_>>()
        .join(" ")
}

fn into_angle_categories(hits: &Vec<math::Hit>) -> BTreeMap<i32, Vec<&math::Hit>> {
    let mut map: BTreeMap<i32, Vec<&math::Hit>> = BTreeMap::new();

    for hit in hits {
        let angle = hit.get_angle();
        let category = (angle / 10) * 10;

        if map.contains_key(&category) {
            map.get_mut(&category).unwrap().push(hit);
        } else {
            map.insert(category, vec![hit]);
        }
    }

    map
}

fn print_info(source: &Option<Cursor>, target: &Option<Cursor>, mode: &Mode) {
    let source = source.clone();
    let target = target.clone();
    let mode = mode.clone();

    println!("[INFO] 1: Save Position 1");
    println!("[INFO] 2: Save Position 2");
    println!("[INFO] 3: Calculate");
    println!("[INFO] 4: Clear Positions");
    println!("[INFO] 5: Switch Calculation Mode");
    println!("[INFO] 6: Clear Console");
    println!();

    if source.is_some() {
        println!("[INFO] Position 1: ({}, {})", source.as_ref().unwrap().get_x(), source.as_ref().unwrap().get_y());
    } else {
        println!("[INFO] Position 1: Not set");
    }

    if target.is_some() {
        println!("[INFO] Position 2: ({}, {})", target.as_ref().unwrap().get_x(), target.as_ref().unwrap().get_y());
    } else {
        println!("[INFO] Position 2: Not set");
    }

    println!("[INFO] Mode: {}", mode);
}