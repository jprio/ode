use std::{ path::Path, io::{ BufWriter, Write }, fs::File };
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{ PointMarker, PointStyle };

use ode_solvers::*;
const OUT_FILE_NAME: &'static str = "outputs/sample.png";

/*
Exemple : https://srenevey.github.io/ode-solvers/examples/kepler_orbit.html
d2x/dt2 = -wx 
avec 
* x(0)=1
* dx(0)/dt=1
 */
fn main() {
    let system = HarmonicOscillator { w: 1.0 };

    let y0 = State::new(1.0, 0.0);

    let mut stepper = Dopri5::new(system, 0.0, 50.0, 0.1, y0, 1.0e-10, 1.0e-10);
    let res = stepper.integrate();

    // Handle result
    match res {
        Ok(stats) => {
            println!("{}", stats);

            // Do something with the output...
            let path = Path::new("./outputs/harmonic_oscillator.dat");
            save(stepper.x_out(), stepper.y_out(), path);
            println!("Results saved in: {:?}", path);
        }
        Err(_) => println!("An error occured."),
    }

    draw_chart(stepper.x_out(), stepper.y_out());
}
type State = Vector2<f64>; // la position et la vitesse, sur un seul axe
type Time = f64;
struct HarmonicOscillator {
    w: f64,
}
impl ode_solvers::System<State> for HarmonicOscillator {
    // Equations of motion of the system
    fn system(&self, _t: Time, y: &State, dy: &mut State) {
        /*
         */
        dy[0] = y[1];
        dy[1] = -self.w * y[0];
    }
}

pub fn save(times: &Vec<Time>, states: &Vec<State>, filename: &Path) {
    // Create or open file
    let file = match File::create(filename) {
        Err(e) => {
            println!("Could not open file. Error: {:?}", e);
            return;
        }
        Ok(buf) => buf,
    };
    let mut buf = BufWriter::new(file);

    // Write time and state vector in a csv format
    for (i, state) in states.iter().enumerate() {
        buf.write_fmt(format_args!("{}", times[i])).unwrap();
        for val in state.iter() {
            buf.write_fmt(format_args!(", {}", val)).unwrap();
        }
        buf.write_fmt(format_args!("\n")).unwrap();
    }
    if let Err(e) = buf.flush() {
        println!("Could not write to file. Error: {:?}", e);
    }
}

fn draw_chart(times: &Vec<Time>, states: &Vec<State>) -> Result<(), Box<dyn std::error::Error>> {
    // Scatter plots expect a list of pairs
    //let data1 = vec![(-3.0, 2.3), (-1.6, 5.3), (0.3, 0.7), (4.3, -1.4), (6.4, 4.3), (8.5, 3.7)];
    //let data1 = std::iter::zip(times, states).collect();
    let mut data1: Vec<(f64, f64)> = Vec::new();

    for (i, state) in states.iter().enumerate() {
        data1.push((times[i], *state.iter().next().unwrap()));
    }
    // We create our scatter plot from the data
    let s1: Plot = Plot::new(data1).point_style(
        PointStyle::new()
            //.marker(PointMarker::Square) // setting the marker to be a square

            .colour("#DD3355")
    ); // and a custom colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .x_range(0.0, 10.0)
        .y_range(-2.0, 2.0)
        .x_label("Some varying variable")
        .y_label("The response of something");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("scatter.svg").unwrap();
    Ok(())
}