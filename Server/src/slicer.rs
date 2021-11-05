use serde::Deserialize;
use std::{
    fs, io,
    io::Read,
    process::{Command, Output},
};

const GCODE_OUTPUT: &str = "superslicer/OpenSCAD Model.gcode";
const SCAD_OUTPUT: &str = "superslicer/model.3mf";

const OPENSCAD: &str = "/Applications/OpenSCAD.app/Contents/MacOS/OpenSCAD";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Fills {
    Rectilinear,
    Monotonic,
    Grid,
    Triangles,
    Stars,
    Line,
    Honeycomb,
    Hexagonal,
    Gyroid,
    Hilbertcurve,
    Archimedeanchords,
    Octagramspiral,
    Scatteredrectilinear,
}
impl Fills {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[derive(Debug, Deserialize)]
pub struct SlicerOptions {
    svg: String,
    fill_density: f32,
    fill_pattern: Fills,
    fill_connected: bool,
    fill_overlap: f32,
    fill_angle: i32,
    fill_speed: i32,
    perimeters: i32,
    perimeter_speed: i32, // perimeter_width: f32,
}
impl SlicerOptions {
    fn args(self) -> String {
        [
            format!("-g"),
            format!("--load slicer-config.ini"),
            format!("--fill-density {}%", self.fill_density * 100.0),
            format!("--fill-pattern {}", self.fill_pattern.to_string()),
            format!(
                "--infill-connection {}",
                if self.fill_connected {
                    "connected"
                } else {
                    "notconnected"
                }
            ),
            format!("--infill-overlap {}%", self.fill_overlap * 100.0),
            format!("--fill-angle {}", self.fill_angle),
            format!("--infill-speed {}", self.fill_speed),
            format!("--perimeters {}", self.perimeters),
            format!("--perimeter-speed {}", self.perimeter_speed),
            SCAD_OUTPUT.to_string(),
        ]
        .join(" ")
    }
}

fn openscad(svg: &str) -> Result<Output, io::Error> {
    println!("Processing OPENSCAD");
    fs::write("drawing.svg", svg)?;
    Command::new(OPENSCAD)
        .arg(format!("-o{}", SCAD_OUTPUT))
        .arg("convert.scad")
        .output()
}

fn superslice(options: SlicerOptions) -> std::io::Result<std::process::Output> {
    println!("Processing SUPERSLICER");
    Command::new("./superslice.sh")
        .env("SARGS", options.args())
        .output()
}

async fn read_gcode() -> Result<String, io::Error> {
    println!("Reading gcode");
    let mut data = String::new();
    let mut f = fs::File::open(GCODE_OUTPUT)?;
    f.read_to_string(&mut data).expect("Unable to read string");
    Ok(data)
}

pub async fn slice(options: SlicerOptions) -> Result<String, io::Error> {
    println!("{:?}", openscad(&options.svg)?);
    println!("{:?}", superslice(options)?);
    Ok(read_gcode().await?)
}
