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
            colour: 7.0,
            mode: Mode::PenUp
        }
    }
}

impl Turtle{
    fn pen_up(&mut self){
        self.mode = Mode::PenUp;
    }
    fn pen_down(&mut self){
        self.mode = Mode::PenDown;
    }
    fn move_turtle(&mut self, numpixels: f32, image:&mut Image) -> Result<(), String>{

        match self.mode {
            Mode::PenUp => {
                self.pen_up_move(numpixels);
                Ok(())
            },
            Mode::PenDown => self.pen_down_move(numpixels, image),
        }
    }
    fn pen_up_move(&mut self, numpixels: f32){

        let angle = self.angle.to_radians();
        let x = (100f32*numpixels*angle.sin()).round() / 100.0;
        let y = (100f32*numpixels*angle.cos()).round() / 100.0;
        let translation_vector = Location::new(x, y);
        self.position = self.position.translate(&translation_vector);

    }
    fn pen_down_move(&mut self, numpixels: f32, image: &mut Image) -> Result<(), String> {
        let (x, y) = match image.draw_simple_line(
            self.position.x()+((image.get_dimensions().0/2) as f32),
            ((image.get_dimensions().1 as f32) - self.position.y())-((image.get_dimensions().1/2) as f32),
            self.angle as i32,
            numpixels,
            COLORS[self.colour as usize]
        ) {
            Ok(tup) => tup,
            Err(error) => {
                return Err(error.to_string());
            }
        };
        self.position = Location::new(x-((image.get_dimensions().0 / 2) as f32),((image.get_dimensions().1 / 2) as f32) - y);
        Ok(())

    }
    fn turn(&mut self, degrees: f32) -> Result<(), String>{
        self.set_heading(self.angle+degrees)
    }

    fn set_heading(&mut self, degrees: f32) -> Result<(), String> {
        if degrees.fract() > 0.0 {
            return Err(String::from("The angle must be an integer"))
        }
        self.angle = degrees;
        Ok(())
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
    // Some of the tests ignoring result from error able functions because the error is not related
    // to the functionality tested and also the values are specified for the test such that it is
    // guaranteed they cant cause an error. There are tests to check the error checking in these
    // functions to ensure they catch errors. Therefore i can ensure that these errors can be
    // ignored in those tests.
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
        let _ = turtle.set_heading(90.0);  
        let _ = turtle.set_heading(45.0);
        assert_eq!(turtle.angle,45.0);
    }
    #[test]
    fn set_heading_valid_large(){
        let mut turtle = Turtle::new();
        let _ = turtle.set_heading(90.0);
        let _ = turtle.set_heading(370.0);
        assert_eq!(turtle.angle,370.0);
    }
    #[test]
    fn set_heading_invalid() {
        let mut turtle = Turtle::new();
        let res = turtle.set_heading(4.5);
        assert_eq!(res.is_err(), true);
        
    }
    #[test]
    fn turn_turtle_valid_small(){
        let mut turtle = Turtle::new();
        let _ = turtle.set_heading(45.0);
        let _ = turtle.turn(90.0);
        assert_eq!(turtle.angle,135.0);
    }
    #[test]
    fn turn_turtle_valid_big(){
        let mut turtle = Turtle::new();
        let _ = turtle.set_heading(10.0);
        let _ = turtle.turn(360.0);
        assert_eq!(turtle.angle,370.0);
    }
    #[test]
    fn turn_turtle_invalid(){
        let mut turtle = Turtle::new();
        let res = turtle.turn(90.5);
        assert_eq!(res.is_err(), true);
    }
    
    #[test]
    fn move_turtle_forward(){
        let mut image = Image::new(64, 64);
        let mut turtle = Turtle::new();
        match turtle.move_turtle(2.0,&mut image) {
            Ok(_) => (),
            Err(_) => panic!("error drawing on image, shouldnt be drawing image though")
        }
        assert_eq!(turtle.position,Location::new(0.0,2.0));
    }

    #[test]
    fn move_turtle_back(){
        let mut image = Image::new(64, 64);
        let mut turtle = Turtle::new();
        let _ = turtle.turn(180.0); // TODO explain why error ignored.
        match turtle.move_turtle(2.0,&mut image) {
            Ok(_) => (),
            Err(_) => panic!("error drawing on image, shouldnt be drawing image though")
        }

        assert_eq!(turtle.position,Location::new(0.0,-2.0));
    }

    #[test]
    fn move_turtle_right(){
        let mut image = Image::new(64, 64);
        let mut turtle = Turtle::new();
        let _ = turtle.turn(90.0);
        match turtle.move_turtle(2.0,&mut image) {
            Ok(_) => (),
            Err(_) => panic!("error drawing on image, shouldnt be drawing image though")
        }
        assert_eq!(turtle.position,Location::new(2.0, 0.0));
    }

    #[test]
    fn move_turtle_left(){
        let mut image = Image::new(64, 64);
        let mut turtle = Turtle::new();
        let _ = turtle.turn(270.0);
        match turtle.move_turtle(2.0,&mut image) {
            Ok(_) => (),
            Err(_) => panic!("error drawing on image, shouldnt be drawing image though")
        }
        assert_eq!(turtle.position,Location::new(-2.0,0.0));
    }

    #[test]
    fn move_turtle_diagonal_small(){
        let mut image = Image::new(64, 64);
        let mut turtle = Turtle::new();
        let _ = turtle.set_heading(45.0);
        match turtle.move_turtle((2f32).sqrt(),&mut image){
            Ok(_) => (),
            Err(_) => panic!("error drawing on image, shouldnt be drawing image though")
        }
    }

    #[test]
    fn move_turtle_diagonal_large(){
        let mut image = Image::new(64, 64);
        let mut turtle = Turtle::new();
        let _ = turtle.set_heading(360f32+45f32);
        match turtle.move_turtle((2f32).sqrt(),&mut image) {
            Ok(_) => (),
            Err(_) => panic!("error drawing on image, shouldnt be drawing image though")
        }
        assert_eq!(turtle.position,Location::new(1f32,1f32));
    }

    #[test]
    fn draw_line(){
        let mut image = Image::new(256,256);
        let mut turtle = Turtle::new();
        turtle.pen_down();
        let _ = turtle.move_turtle(100.0,&mut image);
        let _ = image.save_png("one_line_test.png");
        assert_eq!(turtle.position, Location::new(0.0,100.0));
    }
    #[test]
    fn draw_left() { todo!(); }

    #[test]
    fn draw_right() { todo!(); }

    #[test]
    fn draw_back() { todo!(); }

    #[test]
    fn draw_non_origin(){ todo!(); }

    #[test]
    fn draw_circle(){ todo!(); }

    #[test]
    fn draw_two_lines(){ todo!(); }

    #[test]
    fn more_test(){ todo!()}




}
