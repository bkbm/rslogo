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
            angle: 0f32,
            colour: 7f32,
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
        let x = (1000f32*numpixels*angle.sin()).round() / 1000f32;
        let y = (1000f32*numpixels*angle.cos()).round() / 1000f32;
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
        let x = x-((image.get_dimensions().0 / 2) as f32);
        let y = ((image.get_dimensions().1 / 2) as f32) - y;
        
        let x = (1000f32 * x).round() / 1000f32;
        let y = (1000f32 * y).round() / 1000f32;
        self.position = Location::new(x,y);
        Ok(())

    }
    
    fn turn(&mut self, degrees: f32) -> Result<(), String>{
        self.set_heading(self.angle+degrees)
    }

    fn set_heading(&mut self, degrees: f32) -> Result<(), String> {
        if degrees.fract() > 0f32 {
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
        turtle.set_x(10f32);
        assert_eq!(turtle.position,Location::new(10f32,0f32));
    }

    #[test]
    fn setting_y_position(){
        let mut turtle = Turtle::new();
        turtle.set_y(10f32);
        assert_eq!(turtle.position,Location::new(0f32,10f32));
    }

    #[test]
    fn set_heading_valid_small(){
        let mut turtle = Turtle::new();
        let _ = turtle.set_heading(90f32);  
        let _ = turtle.set_heading(45f32);
        assert_eq!(turtle.angle,45f32);
    }
    
    #[test]
    fn set_heading_valid_large(){
        let mut turtle = Turtle::new();
        let _ = turtle.set_heading(90f32);
        let _ = turtle.set_heading(370f32);
        assert_eq!(turtle.angle,370f32);
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
        let _ = turtle.set_heading(45f32);
        let _ = turtle.turn(90f32);
        assert_eq!(turtle.angle,135f32);
    }
    
    #[test]
    fn turn_turtle_valid_big(){
        let mut turtle = Turtle::new();
        let _ = turtle.set_heading(10f32);
        let _ = turtle.turn(360f32);
        assert_eq!(turtle.angle,370f32);
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
        match turtle.move_turtle(2f32,&mut image) {
            Ok(_) => (),
            Err(_) => panic!("error drawing on image, shouldnt be drawing image though")
        }
        assert_eq!(turtle.position,Location::new(0f32,2f32));
    }

    #[test]
    fn move_turtle_back(){
        let mut image = Image::new(64, 64);
        let mut turtle = Turtle::new();
        let _ = turtle.turn(180f32); // TODO explain why error ignored.
        match turtle.move_turtle(2f32,&mut image) {
            Ok(_) => (),
            Err(_) => panic!("error drawing on image, shouldnt be drawing image though")
        }

        assert_eq!(turtle.position,Location::new(0f32,-2f32));
    }

    #[test]
    fn move_turtle_right(){
        let mut image = Image::new(64, 64);
        let mut turtle = Turtle::new();
        let _ = turtle.turn(90f32);
        match turtle.move_turtle(2f32,&mut image) {
            Ok(_) => (),
            Err(_) => panic!("error drawing on image, shouldnt be drawing image though")
        }
        assert_eq!(turtle.position,Location::new(2f32, 0f32));
    }

    #[test]
    fn move_turtle_left(){
        let mut image = Image::new(64, 64);
        let mut turtle = Turtle::new();
        let _ = turtle.turn(270f32);
        match turtle.move_turtle(2f32,&mut image) {
            Ok(_) => (),
            Err(_) => panic!("error drawing on image, shouldnt be drawing image though")
        }
        assert_eq!(turtle.position,Location::new(-2f32,0f32));
    }

    #[test]
    fn move_turtle_diagonal_small(){
        let mut image = Image::new(64, 64);
        let mut turtle = Turtle::new();
        let _ = turtle.set_heading(45f32);
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
    fn draw_forward(){
        let mut image = Image::new(256,256);
        let mut turtle = Turtle::new();
        turtle.pen_down();
        let _ = turtle.move_turtle(100f32,&mut image);
        let _ = image.save_png("one_forward_test.png");
        assert_eq!(turtle.position, Location::new(0f32,100f32));
    }

    #[test]
    fn draw_left() {
        let mut image = Image::new(256,256);
        let mut turtle = Turtle::new();
        turtle.pen_down();
        let _ = turtle.turn(270f32);
        let _ = turtle.move_turtle(100f32,&mut image);
        let _ = image.save_png("one_left_test.png");
        assert_eq!(turtle.position, Location::new(-100f32,0f32));
    }

    #[test]
    fn draw_right() {
        let mut image = Image::new(256,256);
        let mut turtle = Turtle::new();
        turtle.pen_down();
        let _ = turtle.turn(90f32);
        let _ = turtle.move_turtle(100f32,&mut image);
        let _ = image.save_png("one_right_test.png");
        assert_eq!(turtle.position, Location::new(100f32,0f32));
    }

    #[test]
    fn draw_back() {
        let mut image = Image::new(256,256);
        let mut turtle = Turtle::new();
        turtle.pen_down();
        let _ = turtle.turn(180f32);
        let _ = turtle.move_turtle(100f32,&mut image);
        let _ = image.save_png("one_back_test.png");
        assert_eq!(turtle.position, Location::new(0f32,-100f32));
    }

    #[test]
    fn draw_non_origin() {
        let mut image = Image::new(256,256);
        let mut turtle = Turtle::new();
        let _ = turtle.set_heading(90f32);
        let _ = turtle.move_turtle(50f32,&mut image);
        let _ = turtle.set_heading(0f32);
        turtle.pen_down();
        let _ = turtle.move_turtle(100f32,&mut image);
        let _ = image.save_png("non_origin_test.png");
        assert_eq!(turtle.position, Location::new(50f32,100f32));
    }

    #[test]
    fn draw_diagonal() {
        let mut image = Image::new(256,256);
        let mut turtle = Turtle::new();
        turtle.pen_down();
        let _ = turtle.set_heading(45f32);
        let _ = turtle.move_turtle(100f32,&mut image);
        let _ = image.save_png("one_diagonal_test.png");
        assert_eq!(turtle.position,Location::new(70.711,70.711));
    }

    #[test]
    fn draw_two_lines(){
        let mut image = Image::new(256,256);
        let mut turtle = Turtle::new();
        turtle.pen_down();
        let _ = turtle.move_turtle(50f32,&mut image);
        turtle.pen_up();
        let _ = turtle.move_turtle(20f32,&mut image);
        turtle.pen_down();
        let _ = turtle.move_turtle(50f32,&mut image);
        let _ = image.save_png("two_lines_test.png");
        assert_eq!(turtle.position, Location::new(0f32,120f32));
    }





}
