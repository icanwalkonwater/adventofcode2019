pub use hull_painter::HullPainter;

mod hull_painter;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    None,
    White,
    Black,
}

pub fn start_hull_painter(memory: Vec<i64>) -> [Color; 101 * 101] {
    // 101 x 101
    let mut canvas = [Color::None; 101 * 101];
    let dimensions = (101, 101);
    let mut painter = HullPainter::new(memory, vec![0], (50, 50));

    while !painter.done() {
        painter.round(&mut canvas, &dimensions);
    }

    canvas
}

pub fn start_hull_painter2(memory: Vec<i64>) -> [Color; 101 * 101] {
    let mut canvas = [Color::None; 101 * 101];
    canvas[50 * 101 + 50] = Color::White;
    let dimensions = (101, 101);
    let mut painter = HullPainter::new(memory, vec![1], (50, 50));

    while !painter.done() {
        painter.round(&mut canvas, &dimensions);
    }

    canvas
}
