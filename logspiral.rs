//3d Logarithmic Spiral Example


use turtle::Turtle;

 fn main() {
 
 let mut fval = 0.0;
 let mut turtle = Turtle::new();
	
     for _i in 0..=2048 {
         turtle.forward(fval);
         turtle.right(1.0);
         
	//Adjust the tightness of the spiral ex. 0.0011 - 0.0009
        fval -= 0.001;
           }
	
	turtle.drawing().save_svg("logspiral.svg");
 }
