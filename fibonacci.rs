//Faux-bonacci Spiral Example
//Maybe ot quite a true Fibonacci spiral but pretty close
use turtle::Turtle;

 fn main() {
 
 let mut fval = 0.0;
 let mut turtle = Turtle::new();
	
     for i in 0..=2048 {
         turtle.forward(fval);
         turtle.right(1.0);
         
	//Adjust the tightness of the spiral ex. 0.00001-0.00003
	//Change sign to change direction.
	
        fval += 0.0000168 * i as f64;
        
           }
	
	turtle.drawing().save_svg("fibspiral.svg");
 }
