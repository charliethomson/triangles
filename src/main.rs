

use coffee::{
    graphics::{
        Point,
        Mesh,
        Color,
        Shape,
        WindowSettings,
        Window,
        Frame,
    },
    load::{
        Task,
    },
    Game,
    Timer,
};

const SCL: f32 = 480.0;
const FILL: bool = false;

fn triangle(a: Point, b: Point, c: Point) -> Shape {
    Shape::Polyline { points: vec![a, b, c] }
}

struct Context {
    mesh: Mesh,
    dims: (f32, f32),
    stack: (Point, Point, Point),
    colors: Vec<Color>,
    scale: f32,
    stop: bool,
    iter: usize,
} impl Game for Context {
    type Input = ();
    type LoadingScreen = ();

    fn load(window: &Window) -> Task<Self> {
        let dims = (window.width(), window.height());
        Task::new(move || Context {
            mesh: Mesh::new(),
            dims,
            stack: (
                Point::new(0.0, 0.0),
                Point::new(SCL, 0.0),
                Point::new(0.0, SCL),
            ),
            colors: vec![
                Color::from_rgb(255, 000, 000),
                Color::from_rgb(255, 128, 000),
                Color::from_rgb(255, 255, 000),
                Color::from_rgb(128, 255, 000),
                
                Color::from_rgb(000, 255, 000),
                Color::from_rgb(000, 255, 128),
                Color::from_rgb(000, 255, 255),
                Color::from_rgb(000, 128, 255),

                Color::from_rgb(000, 000, 255),
                Color::from_rgb(128, 000, 255),
                Color::from_rgb(255, 000, 255),
                Color::from_rgb(255, 000, 128),
            ],
            scale: SCL,
            stop: false,
            iter: 0,
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        if self.stop {
            self.iter = 0;
            self.mesh = Mesh::new();
            self.stop = false;
            eprintln!("CLEAR");
            self.stack = (
                Point::new(0.0, 0.0),
                Point::new(SCL, 0.0),
                Point::new(0.0, SCL),
            );
        } else if self.iter % 10 == 0 {

            let (a, b, c) = self.stack;
            
            let mut d = Point::new(0.0, 0.0);
            
            if b.x == 0.0 {
                d = Point::new(0.0, b.y + self.scale);
            } 
            if b.y == 0.0 {
                d = Point::new(b.x + self.scale, 0.0);
            }
            if b.x >= self.dims.0 {
                d = Point::new(self.dims.0, b.y + self.scale);
            } else if b.y >= self.dims.1 {
                d = Point::new(b.x + self.scale, self.dims.1)
            }
            
            if b.x >= self.dims.0 && b.y >= self.dims.1 {
                self.stop = true;
            } else if d.x >= self.dims.0 && d.y >= self.dims.1 {
                if FILL {
                    self.mesh.fill(triangle(a, b, c), self.colors[self.iter % self.colors.len()]);
                    self.mesh.fill(triangle(b, c, Point::new(self.dims.0, self.dims.1)), self.colors[(self.iter + 1) % self.colors.len()]);
                } else {
                    self.mesh.stroke(triangle(a, b, c), self.colors[self.iter % self.colors.len()], 2);
                    self.mesh.stroke(triangle(b, c, Point::new(self.dims.0, self.dims.1)), self.colors[(self.iter + 1) % self.colors.len()], 2);
                }
                self.stop = true;
            }

            if FILL {
                self.mesh.fill(triangle(a, b, c), self.colors[self.iter % self.colors.len()]);
            } else {
                self.mesh.stroke(triangle(a, b, c), self.colors[self.iter % self.colors.len()], 2);
            }

            self.stack = (b, c, d);
            
            self.mesh.draw(&mut frame.as_target());
            
            self.iter += 1;
        } else {
            self.mesh.draw(&mut frame.as_target());
            self.iter += 1;
        }
    }
}




fn main() {
    Context::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1920, 1080),
        resizable: true,
        fullscreen: true,
    }).unwrap();
}
