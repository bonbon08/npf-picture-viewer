use gfxsrc;
use std::env;

fn main() {
 let args : Vec<String> = env::args().collect();
 let patd = &args[1];
 let mut app = gfxsrc::Screen::new(20, 8, ' '.to_string());
 app.set_title("Picture_Viewer", "#FFFFFF");
 app.addoutline("#FFFFFF");
 app.addstring(1, 2, "Height of Image", "#FFFFFF");
 app.addstring(1, 4, "Width of Image", "#FFFFFF");
 let height = app.addinput(1, 3, "==> ", "#FFFFFF");
 let width = app.addinput(1, 5, "==> ", "#FFFFFF");
 let height: usize = height.trim().parse().expect("Not a number!");
 let width: usize = width.trim().parse().expect("Not a number!");

let patd = patd.trim();
println!("File opened successfully");
app.cls();
app.updatewindow(width+2, height+2, '#'.to_string());
match app.addpic(0, 0, &patd) {
      Ok(_) => println!(""),
      Err(e) => println!("Failed to show picture: {}", e),
   }
app.print();
}

