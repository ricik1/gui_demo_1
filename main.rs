//ass our lib 
//use sdl2::{pixels::Color, rect::Rect, video::{Window, WindowBuilder}, AudioSubsystem};
use sdl2::{pixels::Color, rect::Rect, video::Window};

use std::time::Duration;

//creat our main fn
fn main() {

    //use our sdl lib to create a video subsystem
    //sdl stands for simple direct media layer
   let sld_context = sdl2::init().unwrap();  //check to be sure this right
    // create a window to draw on the window
    let video_subsystem = sld_context.video().unwrap();
    let mut window = video_subsystem
    .window("Game", 800, 600) 
    .position_centered()
    .build()
    .unwrap();


    //create a canvas to draw on
   // let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
   let mut canvas = window.into_canvas().build().unwrap();

// I changed the colors to be able to tell which line of code is for which item
    //horzonal wall
    // creat a width for our wall
    let width: u32 = 750;
    //create a height for wall
    let height: u32 = 20;
    // create a square
    let square = Rect::new(25, 100, width, height);
    //blue color
    canvas.set_draw_color(Color::RGB(0, 0, 255));
    //fill up the square
    canvas.fill_rect(square).unwrap();
    //draw a dot in the center of the square
    //create a var for the dot size
    let dot_size:u32 = 10;
    //creat a dot struct that contains the position and size of our dot
    let dot_rect = Rect::new(
        width as i32,
        350 as i32,
         dot_size,
         dot_size,

    );


    // set the draw color, and we want to do this every time we draw something
    //purple color
    canvas.set_draw_color(Color::RGB(128,0, 128));
    //fill the dot
    canvas .fill_rect(dot_rect).unwrap();
    //draw a dot in the center of the square
    //create a var for the  dot soze
    let dot2_size:u32 =50;
    //creat a dot struct that contains the postion and the size of sot
    let dot_rect:Rect = Rect::new (200 as i32,
         200 as i32,
        dot2_size,
        dot2_size
    
    );
      
        //added dots in all different location on the page
            // set the draw color, and we wamnt to dot his every time we draw something
            //color blue
    canvas.set_draw_color(Color::RGB(0,0,255));
    //fill the dot
    canvas .fill_rect(dot_rect).unwrap();
    //draw a dot in the center of the square
    //create a var for the  dot soze
    let dot2_size:u32 =30;
    //creat a dot struct that contains the postion and the size of sot
    let dot_rect:Rect = Rect::new (300 as i32,
         550 as i32,
        dot2_size,
        dot2_size
    
    );
        //added dots in all different location on the page
            // set the draw color, and we wamnt to dot his every time we draw something
            //color purple
            canvas.set_draw_color(Color::RGB(128,0, 128));
            //fill the dot
            canvas .fill_rect(dot_rect).unwrap();
            //draw a dot in the center of the square
            //create a var for the  dot soze
            let dot2_size:u32 =20;
            //creat a dot struct that contains the postion and the size of sot
            let dot_rect:Rect = Rect::new (400 as i32,
                 350 as i32,
                dot2_size,
                dot2_size
            
            );
                //added dots in all different location on the page
            // set the draw color, and we wamnt to dot his every time we draw something
            //color---green
    canvas.set_draw_color(Color::RGB(255,0, 0));
    //fill the dot
    canvas .fill_rect(dot_rect).unwrap();
    //draw a dot in the center of the square
    //create a var for the  dot soze
    let dot2_size:u32 =40;
    //creat a dot struct that contains the postion and the size of sot
    let dot_rect:Rect = Rect::new (200 as i32,
         450 as i32,
        dot2_size,
        dot2_size
    
    ); 
        //added dots in all different location on the page
            // set the draw color, and we wamnt to dot his every time we draw something
            //color turqoiuse 
            canvas.set_draw_color(Color::RGB(0,255, 0));
            //fill the dot
            canvas .fill_rect(dot_rect).unwrap();
            //draw a dot in the center of the square
            //create a var for the  dot soze
            let dot2_size:u32 =40;
            //creat a dot struct that contains the postion and the size of sot
            let dot_rect:Rect = Rect::new (40 as i32,
                 250 as i32,
                dot2_size,
                dot2_size
            
            );
    
       
    //set the draw color and we want to so this every time we draw
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    //fill the dot
    canvas.fill_rect(dot_rect).unwrap();

//draw the canvas on the window
canvas.present();
    //loop until the user closes the window by pressing escape
    let mut event_pump = sld_context.event_pump().unwrap();
        'running: loop{
            for event in event_pump.poll_iter(){
                match event{
                    sdl2::event::Event::Quit { .. }| 
                    sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::Escape),
                        ..
                    } => break 'running,
                
                   _=> {}
                }
            }
            std::thread::sleep(Duration::from_millis(16));
            }
        }
      
    





