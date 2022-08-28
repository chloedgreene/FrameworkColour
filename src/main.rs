use std::{process::Command, env, str::FromStr};

#[derive(PartialEq)]
enum Colour {
    Off,
    White,
    Red,
    Amber,
    Yellow,
    Blue,
    Green,
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Power,
}

impl FromStr for Direction{

    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "left" => Ok(Direction::Left),
            "right" => Ok(Direction::Right),
            "power" => Ok(Direction::Power),
            _      => Err(()),
        }
    }
}

impl FromStr for Colour{

    type Err = ();

    fn from_str(input: &str) -> Result<Colour, Self::Err> {
        match input {

            "off"  => Ok(Colour::Off),
            "white"    => Ok(Colour::White),
            "red"    => Ok(Colour::Red),
            "amber"  => Ok(Colour::Amber),
            "yellow"  => Ok(Colour::Yellow),
            "blue" => Ok(Colour::Blue),
            "green"   => Ok(Colour::Green),

            _      => Err(()),
        }
    }
}


fn main() {

    let args: Vec<String> = env::args().collect();

    //number 1 is just the path
    if args.len() != 3{
        panic!("I NEED METH JESSE(please make sure you have 2 arguments)")
    }
    if args.len() == 1{
        println!("frameworkcolour [left,right,power] [off,white,red,amber,yellow,blue,green]");
    }



    change_colour(Direction::from_str(&args[1]).expect("Unknown Or Invalid Direction"), Colour::from_str(&args[2]).expect("Unknown Or Invalid Colour"), false);
}

fn change_colour(pos: Direction, led_colour: Colour, gui: bool) {
    //no_blue_led in power
    if pos == Direction::Power && led_colour == Colour::Blue {
        panic!("No Blue Led in power")
    }

    let direction = match pos {
        Direction::Left => "left",
        Direction::Right => "right",
        Direction::Power => "power",
    };

    //messy code here
    let colour = match led_colour {
        Colour::Off => "off",
        Colour::White => "white",
        Colour::Red => "red",
        Colour::Amber => "amber",
        Colour::Yellow => "yellow",
        Colour::Blue => "blue",
        Colour::Green => "green",
    };

    let program = match gui {
        true => "pkexec",
        false => "sudo",
    };

    let args = format!("./tools/ectool led {} {}", direction, colour);

    Command::new(program)
        .args(args.split(" "))
        .status()
        .expect("error running code");
}
