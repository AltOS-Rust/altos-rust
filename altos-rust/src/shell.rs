/*
 * Copyright (C) 2017 AltOS-Rust Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use cortex_m0::kernel;
use cortex_m0::io;
use cortex_m0::time::delay_ms;
use kernel::task::{TaskHandle, Priority};
use kernel::task::args::{ArgsBuilder, Args};
use kernel::collections::{Vec, String};

const HELP: &'static str = r#"Available Commands:
    echo
	clear
    help
"#;

const CLEAR: &'static str = "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n";

pub fn shell(_args: &mut Args) {
	let mut blink_handle: Option<TaskHandle> = None;
    loop {
        print!(" > ");
        let line = read_line();
        let mut words: Vec<&str> = line.split(' ').collect();
		if words.len() > 0 {
			match words.remove(0) {
				"echo" => {
					for word in words {
						print!("{} ", word);
					}
					println!("");
				},
				"clear" => {
					println!("{}", CLEAR);
					println!("{}", CLEAR);
				},
				"blink" => {
                    let rate: usize = if words.len() > 0 {
                        words[0].parse::<usize>().unwrap_or(100)
                    }
                    else {
                        100
                    };

					if let Some(mut handle) = blink_handle.take() {
						handle.destroy();
						turn_off_led();
					}
					let mut args = ArgsBuilder::with_capacity(1);
					args.add_num(rate);
					blink_handle = Some(kernel::syscall::new_task(blink, args.finalize(), 1024, Priority::Low, "blink"));
				},
				"stop" => {
					if let Some(mut handle) = blink_handle.take() {
						handle.destroy();
						turn_off_led();
					}
				},
				"help" => print!("{}", HELP),
				command_word => println!("Unknown command: '{}'", command_word),
			}
		}
    }
}

fn turn_on_led() {
	use cortex_m0::peripheral::gpio::{self, Port};
	let pb3 = Port::new(3, gpio::Group::B);
	pb3.set();
}

fn turn_off_led() {
	use cortex_m0::peripheral::gpio::{self, Port};
	let pb3 = Port::new(3, gpio::Group::B);
	pb3.reset();
}

fn blink(args: &mut Args) {
	use cortex_m0::peripheral::gpio::{self, Port};

	let rate = args.pop_num();
	loop {
		turn_on_led();
		delay_ms(rate);
		turn_off_led();
		delay_ms(rate);
	}
}

fn parse_command(mut words: Vec<&str>) {
}

fn get_and_echo_char() -> Option<char> {
    io::poll_char().map(|ch| {
        print!("{}", ch as char);
        // 8 is \b, but Rust doesn't recognize \b?
        if ch == b'\x08' {
            print!(" ");
            print!("{}", ch as char);
        }
        ch as char
    })
}

fn read_line() -> String {
    let mut line = String::new();
    loop {
        if let Some(ch) = get_and_echo_char() {
            if ch == '\n' || ch == '\r' {
                return line;
            }
            if ch == '\x08' {
                line.pop();
            }
            else {
                line.push(ch);
            }
        }
    }
}
