use unsvg::Image;
use unsvg::COLORS;
use crate::location::Location;
#[derive(Debug)]
pub enum Mode {
    PenUp,
    PenDown
}

#[derive(Debug)]
pub struct Turtle{
    pub position: Location,
    pub angle: f32,
    pub colour: f32,
    pub mode: Mode
}
impl Turtle {
    pub fn new() -> Self{
        Turtle{
            position: Location::origin(),
            angle: 0.0,
            colour: 0.0,
            mode: Mode::PenUp
        }
    }
}

impl Turtle{

    fn method_for_moving_and_drawing_turtle_mode_dependent_needs_name(&mut self, numpixels: f32, image:&mut Image){
        match self.mode {
            Mode::PenUp => self.move_turtle(numpixels),
            Mode::PenDown => self.draw(numpixels, image),
        }
    }
    fn move_turtle(&mut self, numpixels: f32){
        let angle = self.angle.to_radians();
        let x = (100f32*angle.sin()).trunc() / 100.0;
        let y = (100f32*angle.cos()).trunc() / 100.0;
        let translation_vector = Location::new(numpixels*x, numpixels*y);
        self.position = self.position.translate(&translation_vector);

    }
    fn draw(&mut self, numpixels: f32, image: &mut Image) {
        let (x, y) = match image.draw_simple_line(
            self.position.x(),
            self.position.y(),
            self.angle as i32,
            numpixels,
            COLORS[self.colour as usize]
        ) {
            Ok(tup) => tup,
            Err(_) => panic!("Error Drawing")
        };
        self.position = Location::new(x,y);

    }
    fn turn(&mut self, degrees: f32) {
        self.set_heading(self.angle+degrees);
    }

    fn set_heading(&mut self, degrees: f32) {
        if degrees.fract() > 0.0 {
            panic!("Only Integer Degrees are allowed!")
        }
        self.angle = degrees;
    }

    fn change_colour(&mut self, colourcode: f32) {
        self.colour = colourcode;
    }

    fn set_x(&mut self, numpixels: f32){
        self.position = Location::new(numpixels, self.position.y());
    }

    fn set_y(&mut self, numpixels: f32){
        self.position = Location::new(self.position.x(), numpixels);
    }

}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn setting_x_position(){
        let mut turtle = Turtle::new();
        turtle.set_x(10.0);
        assert_eq!(turtle.position,Location::new(10.0,0.0));
    }

    #[test]
    fn setting_y_position(){
        let mut turtle = Turtle::new();
        turtle.set_y(10.0);
        assert_eq!(turtle.position,Location::new(0.0,10.0));
    }

    #[test]
    fn set_heading_valid_small(){
        let mut turtle = Turtle::new();
        turtle.set_heading(90.0);
        turtle.set_heading(45.0);
        assert_eq!(turtle.angle,45.0);
    }
    #[test]
    fn set_heading_valid_large(){
        let mut turtle = Turtle::new();
        turtle.set_heading(90.0);
        turtle.set_heading(370.0);
        assert_eq!(turtle.angle,370.0);
    }
    #[test]
    fn set_heading_invalid() {
        let mut turtle = Turtle::new();
        turtle.set_heading(4.5);
        todo!();
    }
    #[test]
    fn turn_turtle_valid_small(){
        let mut turtle = Turtle::new();
        turtle.set_heading(45.0);
        turtle.turn(90.0);
        assert_eq!(turtle.angle,135.0);
    }
    #[test]
    fn turn_turtle_valid_big(){
        let mut turtle = Turtle::new();
        turtle.set_heading(10.0);
        turtle.turn(360.0);
        assert_eq!(turtle.angle,370.0)
    }
    #[test]
    fn turn_turtle_invalid(){
        let mut turtle = Turtle::new();
        turtle.turn(90.5);
        todo!();
    }
    
    #[test]
    fn move_turtle_forward(){
        let mut turtle = Turtle::new();
        turtle.move_turtle(2.0);
        assert_eq!(turtle.position,Location::new(0.0,2.0));
    }

    #[test]
    fn move_turtle_back(){
        let mut turtle = Turtle::new();
        turtle.turn(180.0);
        turtle.move_turtle(2.0);
        assert_eq!(turtle.position,Location::new(0.0,-2.0));
    }

    #[test]
    fn move_turtle_right(){
        let mut turtle = Turtle::new();
        turtle.turn(90.0);
        turtle.move_turtle(2.0);
        assert_eq!(turtle.position,Location::new(2.0, 0.0));
    }

    #[test]
    fn move_turtle_left(){
        let mut turtle = Turtle::new();
        turtle.turn(270.0);
        turtle.move_turtle(2.0);
        assert_eq!(turtle.position,Location::new(-2.0,0.0));
    }

    #[test]
    fn move_turtle_diagonal_small(){
        let mut turtle = Turtle::new();
        turtle.set_heading(45.0);
        turtle.move_turtle((2f32).sqrt());
    }

    #[test]
    fn move_turtle_diagonal_large(){
        let mut turtle = Turtle::new();
        turtle.set_heading(360f32+45f32);
        turtle.move_turtle((2f32).sqrt());
    }

    #[test]
    fn draw_line(){
        todo!();
    }

    #[test]
    fn draw_circle(){ todo!(); }

    #[test]
    fn draw_two_lines(){ todo!(); }

    #[test]
    fn more_test(){ todo!()}




}
