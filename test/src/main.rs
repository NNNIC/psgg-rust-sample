#[allow(dead_code)]
mod hoge_control;
use hoge_control::hoge_control::*;

fn main() {
    let mut hc = HogeControl::new();
    hc.run();
}