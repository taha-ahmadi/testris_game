extern crate termion;

use termion::color;


fn main(){
    println!("{red}more red than any comrade{reset} test and {yellow} yellow now",
    red   = color::Fg(color::Red),
    reset = color::Fg(color::Reset),
    yellow = color::Fg(color::Yellow));
}