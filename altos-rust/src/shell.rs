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
use cortex_m0::time::{delay_ms, now};
use kernel::task::{TaskHandle, Priority};
use kernel::task::args::{ArgsBuilder, Args};
use kernel::collections::{Vec, String};
use kernel::alloc::Box;

const HELP: &'static str = "Available Commands:
    echo
    clear
    eval
    blink
    stop
    uptime
    exit
    help";


enum Expr {
    Op(Box<Expr>, Operator, Box<Expr>),
    Val(isize),
    Invalid(&'static str),
}

impl Expr {
    fn eval(&self) -> ::core::result::Result<isize, &'static str> {
        match *self {
            Expr::Op(ref lhs, ref op, ref rhs) => {
                match (lhs.eval(), rhs.eval()) {
                    (Ok(lhs), Ok(rhs)) => Ok(op.apply(lhs, rhs)),
                    _ => Err("Invalid expression"),
                }
            },
            Expr::Val(x) => Ok(x),
            Expr::Invalid(msg) => Err(msg),
        }
    }
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn apply(&self, lhs: isize, rhs: isize) -> isize {
        match *self {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
        }
    }
}

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
                    print!("{}[2J", 27 as char)
                },
                "eval" => {
                    if words.len() > 2 {
                        let expr = match (words[0].parse(), words[2].parse()) {
                            (Ok(x), Ok(y)) => {
                                match words[1] {
                                    "+" => Expr::Op(Box::new(Expr::Val(x)), Operator::Add, Box::new(Expr::Val(y))),
                                    "-" => Expr::Op(Box::new(Expr::Val(x)), Operator::Sub, Box::new(Expr::Val(y))),
                                    "*" => Expr::Op(Box::new(Expr::Val(x)), Operator::Mul, Box::new(Expr::Val(y))),
                                    "/" => Expr::Op(Box::new(Expr::Val(x)), Operator::Div, Box::new(Expr::Val(y))),
                                    _ => Expr::Invalid("Invalid operator"),
                                }
                            },
                            (Err(_), Ok(_)) => Expr::Invalid("Left expression failed to parse"),
                            (Ok(_), Err(_)) => Expr::Invalid("Right expression failed to parse"),
                            (Err(_), Err(_)) => Expr::Invalid("Both expressions failed to parse"),
                        };
                        match expr.eval() {
                            Ok(result) => println!("{} {} {} = {}", words[0], words[1], words[2], result),
                            Err(msg) => println!("{}", msg),
                        }
                    }
                    else {
                        println!("USAGE: eval <lhs> <op> <rhs>");
                    }
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
                "uptime" => {
                    let hms = uptime();
                    println!("{:02}:{:02}:{:02}", hms.0, hms.1, hms.2);
                },
                "stop" => {
                    if let Some(mut handle) = blink_handle.take() {
                        handle.destroy();
                        turn_off_led();
                    }
                },
                "exit" => kernel::syscall::exit(),
                "help" => println!("{}", HELP),
                "" => {},
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
    let rate = args.pop_num();
    loop {
        turn_on_led();
        delay_ms(rate);
        turn_off_led();
        delay_ms(rate);
    }
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
                // Force the cursor to stay past the end of our prompt
                if let None = line.pop() {
                    print!(" ");
                }
            }
            else {
                line.push(ch);
            }
        }
    }
}

fn uptime() -> (usize, usize, usize) {
    let curr_time = now();

    let mut minutes = curr_time.sec / 60;
    let hours = minutes / 60;
    let seconds = curr_time.sec % 60;
    minutes = minutes % 60;

    (hours, minutes, seconds)
}
