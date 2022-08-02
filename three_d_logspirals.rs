//3d Logarithmic Spiral Example


use turtle::Turtle;

 fn main() {
 
 let mut fval = 0.0;
 let mut tval = 1.0;
 let mut turtle = Turtle::new();
	
	for j in 0..=3 {
     for _i in 0..=8192 {
         turtle.forward(fval);
         turtle.right(tval);
         
	//Adjust the tightness of the spiral
        fval -= 0.0009;
           }
	 turtle.right(90.0);
	 turtle.forward(0.1);
	 turtle.right(90.0);
	 
	 for _i in 0..=8192 {
         turtle.forward(fval);
         turtle.left(tval);
         
	//Adjust the tightness of the spiral
        fval += 0.0009;
       
           }
          turtle.left(90.0);
	 turtle.forward(0.1);
	 turtle.left(90.0); 
	 }
	
	turtle.drawing().save_svg("threedlogspiral1.svg");
 }
