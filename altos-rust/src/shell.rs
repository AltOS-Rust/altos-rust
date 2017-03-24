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
use core::fmt::{self, Display};

/*
const LOGO: &'static str = "
            .
            ;'                ..      '.
           .cc.              x0kk.   .Oo  .xc
           c,;:            .OO. xO.  .Oo 'dkxoc .d.  .o' .clccc.
          ;'  ;'           kKdodd0k. .Oo  .kl   ,k,  'x; ,dc;,'.
         ..    .          cXl   .o0o .Oo  .ko   ,k;  ;x;    ..ld.
         c:.  .::         oO.     dx  xc   cddo, :dolcd' ;lcclc,
        ;c;,',':c'
       .cc,    :cc.
      .ccl;    cll:        'dl   ;o;            ..
      :odxc    dxdd;       kK0d cOkO    .''.    ok    ...    .    .   .....  ...
     ,xkO0o    k00Ox.     .Kx.k0O':O, 'x'  'k:lookol.oxcx;. ;d.  ,d' ,oc.'ll:.'l:
    .k0KKKd    OXKK0k     ,Xl  l   O: xOoccccc  ck.  ox  '' ;d.  ,d' ,o.  :l.  :l
    d0KKXXd    OXXXKKl    ;Xl      Oc 'kd;      :k   ox     'dc  'd' ,o.  :l.  :l
  ;x0KXXO;      cKXXKKx,  xl.      0c.  lcccc   :l   ox      'cccc'   0   :l.  :l
.lxOKKXo          kXKK0ko.                                                          ";
*/


const HELP: &'static str = "Available Commands:
    echo [string ...]
    clear
    eval <lhs> <op> <rhs>
    blink [rate]
    stop
    uptime
    rocket [timer]
    uname
    exit
    help [cmd]";

const ECHO_HELP: &'static str = "Echo a string to the terminal";
const CLEAR_HELP: &'static str = "Clear the terminal";
const EVAL_HELP: &'static str = "Evaluate an expression of the form x <op> y";
const BLINK_HELP: &'static str = "Blink the LED at the given rate in milliseconds";
const STOP_HELP: &'static str = "Stop blinking the LED";
const UPTIME_HELP: &'static str = "Display how long the system has been running as HH:MM:SS";
//const ROCKET_HELP: &'static str = "Deploys a rocket?";
const UNAME_HELP: &'static str = "Displays system information";
const EXIT_HELP: &'static str = "Exit the shell";
const HELP_HELP: &'static str = "Display available commands or more information about a certain command";

enum ReadError {
    UnclosedString,
}

impl Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            ReadError::UnclosedString => "unclosed string found",
        };
        write!(f, "{}", msg)
    }
}

enum Command<'a> {
    Echo,
    Clear,
    Eval,
    Blink,
    Stop,
    Uptime,
    //Rocket,
    Uname,
    Exit,
    Help,
    Invalid(&'a str),
}

impl<'a> Command<'a> {
    fn help_msg(&self) -> (&'static str, &str) {
        match *self {
            Command::Echo => (ECHO_HELP, ""),
            Command::Clear => (CLEAR_HELP, ""),
            Command::Eval => (EVAL_HELP, ""),
            Command::Blink => (BLINK_HELP, ""),
            Command::Stop => (STOP_HELP, ""),
            Command::Uptime => (UPTIME_HELP, ""),
            //Command::Rocket => (ROCKET_HELP, ""),
            Command::Uname => (UNAME_HELP, ""),
            Command::Exit => (EXIT_HELP, ""),
            Command::Help => (HELP_HELP, ""),
            Command::Invalid(invalid) => ("Unknown command: ", invalid),
        }
    }
}

impl<'a> From<&'a str> for Command<'a> {
    fn from(string: &'a str) -> Command<'a> {
        match string {
            "echo" => Command::Echo,
            "clear" => Command::Clear,
            "eval" => Command::Eval,
            "blink" => Command::Blink,
            "stop" => Command::Stop,
            "uptime" => Command::Uptime,
            //"rocket" => Command::Rocket,
            "uname"=> Command::Uname,
            "exit" => Command::Exit,
            "help" => Command::Help,
            invalid => Command::Invalid(invalid),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Token {
    Number(isize),
    Op(Operator),
    LeftParen,
    RightParen,
    EOF,
}

#[derive(Copy, Clone, Debug)]
enum LexError {
    InvalidToken
}

struct Lexer;

impl Lexer {
    fn lex(string: Vec<&str>) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        for thing in string {
            match thing {
                "+" => tokens.push(Token::Op(Operator::Add)),
                "-" => tokens.push(Token::Op(Operator::Sub)),
                "*" => tokens.push(Token::Op(Operator::Mul)),
                "/" => tokens.push(Token::Op(Operator::Div)),
                "(" => tokens.push(Token::LeftParen),
                ")" => tokens.push(Token::RightParen),
                literal => {
                    if let Ok(num) = literal.parse::<isize>() {
                        tokens.push(Token::Number(num));
                    }
                    else {
                        return Err(LexError::InvalidToken);
                    }
                }
            }
        }
        tokens.push(Token::EOF);
        Ok(tokens)
    }
}

#[derive(Copy, Clone, Debug)]
enum ParseError {
    UnexpectedToken,
    UnmatchedParens,
    InvalidOperator,
}

// Grammar:
//
// expression := term
// term := factor ( ( "-" | "+" ) factor )*
// factor := primary ( ( "*" | "/" ) primary )*
// primary := NUMBER | "(" expression ")"
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    fn matches(&mut self, op: Operator) -> bool {
        if self.is_at_end() {
            false
        }
        else {
            match self.peek() {
                Token::Op(operator) if op == operator => {
                    self.advance();
                    true
                },
                _ => false
            }
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek() == Token::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }

    fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.matches(Operator::Add) || self.matches(Operator::Sub) {
            let operator = if let Token::Op(op) = self.previous() {
                op
            }
            else {
                return Err(ParseError::InvalidOperator);
            };
            let right = self.factor()?;
            expr = Expr::Op(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        while self.matches(Operator::Mul) || self.matches(Operator::Div) {
            let operator = if let Token::Op(op) = self.previous() {
                op
            }
            else {
                return Err(ParseError::InvalidOperator);
            };
            let right = self.primary()?;
            expr = Expr::Op(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.advance() {
            Token::Number(num) => Ok(Expr::Val(num)),
            Token::LeftParen => {
                let expr = self.expression()?;
                if let Token::RightParen = self.advance() {
                    Ok(expr)
                }
                else {
                    Err(ParseError::UnmatchedParens)
                }
            },
            _ => Err(ParseError::UnexpectedToken),
        }
    }
}

enum Expr {
    Op(Box<Expr>, Operator, Box<Expr>),
    Val(isize),
}

impl Expr {
    fn eval(&self) -> isize {
        match *self {
            Expr::Op(ref lhs, ref op, ref rhs) => op.apply(lhs.eval(), rhs.eval()),
            Expr::Val(x) => x,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
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
        let line = match read_line() {
            Ok(line) => line,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            },
        };
        let mut words: Vec<&str> = line.iter().map(|s| s.as_ref()).collect();
        //let mut words: Vec<&str> = line.split(' ').collect();
        if words.len() > 0 {
            match Command::from(words.remove(0)) {
                Command::Echo => {
                    for word in words {
                        print!("{} ", word);
                    }
                    println!("");
                },
                Command::Clear => {
                    // ANSI ESC sequence to clear screen and put cursor at at top of terminal.
                    print!("\x1b[2J")
                },
                Command::Eval => {
                    eval(words);
                },
                Command::Blink => {
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
                Command::Stop => {
                    if let Some(mut handle) = blink_handle.take() {
                        handle.destroy();
                        turn_off_led();
                    }
                },
                Command::Uptime => {
                    let hms = uptime();
                    println!("{:02}:{:02}:{:02}", hms.0, hms.1, hms.2);
                },
                /*
                Command::Rocket => {
                    let timer = if words.len() > 0 {
                        words[0].parse::<isize>().unwrap_or(5)
                    }
                    else {
                        5
                    };
                    rocket(timer);
                },
                */
                Command::Uname => {
                    //println!("{}\n", LOGO);
                    //Find more info and place it here
                    println!("AltOS Rust");
                },
                Command::Exit => kernel::syscall::exit(),
                Command::Help => {
                    if words.len() > 0 {
                        let command = Command::from(words[0]);
                        let msg = command.help_msg();
                        println!("{}{}", msg.0, msg.1);
                    }
                    else {
                        println!("{}", HELP);
                    }
                }
                Command::Invalid(invalid) => println!("Unknown command: '{}'", invalid),
            }
        }
    }
}

fn eval(args: Vec<&str>) {
    let tokens = match Lexer::lex(args) {
        Ok(tokens) => tokens,
        Err(err) => {
            println!("Lexing failed: {:?}", err);
            return;
        },
    };
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(expr) => println!("Result: {}", expr.eval()),
        Err(err) => println!("Parsing failed: {:?}", err),
    }
}

fn turn_on_led() {
    use cortex_m0::peripheral::gpio::{self, Port};
    let mut pb3 = Port::new(3, gpio::Group::B);
    pb3.set();
}

fn turn_off_led() {
    use cortex_m0::peripheral::gpio::{self, Port};
    let mut pb3 = Port::new(3, gpio::Group::B);
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

fn read_line() -> Result<Vec<String>, ReadError> {
    let mut line = Vec::new();
    let mut word = String::new();
    let mut in_string = false;
    loop {
        if let Some(ch) = get_and_echo_char() {
            match ch {
                '\n' | '\r' => {
                    if in_string {
                        return Err(ReadError::UnclosedString);
                    }
                    line.push(word);
                    return Ok(line.into_iter().filter(|word| !word.is_empty()).map(|word| word.replace("\"", "")).collect());
                },
                '\x08' => {
                    match word.pop() {
                        Some(ch) if ch == '"' => in_string = !in_string,
                        Some(_) => {},
                        None => match line.pop() {
                            Some(old_word) => word = old_word,
                            None => print!(" "),
                        },
                    }
                },
                ' ' => {
                    if !in_string {
                        line.push(word);
                        word = String::new();
                    }
                    else {
                        word.push(' ');
                    }
                },
                '"' => {
                    word.push('"');
                    in_string = !in_string;
                },
                _ => word.push(ch),
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

/*
fn rocket(mut timer: isize) {
    let mut offset: isize = 15;
    let stationary: isize = offset;
    let counter = timer + offset * 2;
    let mut k = 0;

    if timer < 0 {
        timer = 5;
    }

    let mut rocket = String::new();

    while k < counter {
        rocket.clear();

        print!("\x1b[2J");

        newline_offset(&mut rocket, offset);

        build_rocket_part(&mut rocket, "      /\\\n", offset);
        build_rocket_part(&mut rocket, "     /  \\\n", offset+1);
        build_rocket_part(&mut rocket, "    /    \\\n", offset+2);
        build_rocket_part(&mut rocket, "    |  A |\n", offset+3);
        build_rocket_part(&mut rocket, "    |  L |\n", offset+4);
        build_rocket_part(&mut rocket, "   /|  T |\\\n", offset+5);
        build_rocket_part(&mut rocket, "  / |  O | \\\n", offset+6);
        build_rocket_part(&mut rocket, "  ^^|  S |^^\n", offset+7);
        build_rocket_part(&mut rocket, "    |    |\n", offset+8);

        if (offset % 2 == 0) && (offset < stationary) {
            build_rocket_part(&mut rocket, "     vwwv", offset+9);
        }

        newline_offset(&mut rocket, stationary - offset);

        print!("{}", rocket);

        if timer < 0 {
            if offset > -9 {
                offset -= 1;
            }
            println!("\t\t/-------------\\     Blast off!");
            delay_ms(100);
        }
        else {
            println!("\t\t/-------------\\     Blast off in...{}", timer);
            delay_ms(1000);
        }

        k += 1;
        timer -= 1;
    }
}

fn newline_offset(string: &mut String, offset: isize){
    let mut i = 0;
    if offset < 1 {
        return;
    }
    while i < offset{
        string.push_str("\n");
        i += 1;
    }
}

fn build_rocket_part(rocket: &mut String,part: &str, offset: isize) {
    if offset < 0 {
        return;
    }
    rocket.push_str("\t\t");
    rocket.push_str(part);
}
*/
