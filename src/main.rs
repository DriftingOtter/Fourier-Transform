use std::f64::consts::PI;
use num_complex::Complex;
use plotly::common::Mode;
use plotly::{Plot, Scatter};
use std::env;

const DOMAIN_START: isize = -50;
const DOMAIN_END: isize   = 50;

const AMPLITUDE: f64 = 1.0;
const X_SHIFT: f64 = 0.0;
const Y_SHIFT: f64 = 0.0;

fn main() {
    let _args = match get_command_line_args() {
        Ok(x) => x,
        Err(_) => ("".to_string(), "".to_string()),
    };

    let signal_a = match gen_sine_signal((DOMAIN_START, DOMAIN_END), AMPLITUDE, 261.63, X_SHIFT, Y_SHIFT) {
        Some(x) => x,
        None => {
            eprintln!("Could not generate sine signal.");
            return;
        },
    };

    let signal_b = match gen_sine_signal((DOMAIN_START, DOMAIN_END), AMPLITUDE, 329.63, X_SHIFT, Y_SHIFT) {
        Some(x) => x,
        None => {
            eprintln!("Could not generate cosine signal.");
            return;
        },
    };

    let signal_c = match gen_sine_signal((DOMAIN_START, DOMAIN_END), AMPLITUDE, 392.00, X_SHIFT, Y_SHIFT) {
        Some(x) => x,
        None => {
            eprintln!("Could not generate cosine signal.");
            return;
        },
    };


    let input_signal: Vec<(isize, f64)> = (DOMAIN_START..=DOMAIN_END)
        .map(|j| {
            let index = j.abs() as usize;
            let composite_signal = signal_a[index].1 + signal_b[index].1 + signal_c[index].1;
            (j, composite_signal)
        })
        .collect();

    let transformed_signal = match discrete_fourier_transform(&input_signal, (DOMAIN_START, DOMAIN_END)) {
        Some(x) => x,
        None => {
            eprintln!("Could not perform Fourier transform.");
            return;
        }
    };

    plot_signal("Original Signal", &input_signal);
    plot_signal("Transformed Signal", &transformed_signal);
}

fn plot_signal(title: &str, signal: &[(isize, f64)]) {
    let x_vals: Vec<_> = signal.iter().map(|(x, _)| *x as f64).collect();
    let y_vals: Vec<_> = signal.iter().map(|(_, y)| *y).collect();

    let trace = Scatter::new(x_vals, y_vals).mode(Mode::LinesMarkers).name(title);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
}

fn get_command_line_args() -> Result<(String, String), ()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Incorrect number of command-line arguments. Usage: Fourier_Transform <Signal_Array> <Output_Image_Path>");
        return Err(());
    }
    return Ok((args[1].clone(), args[2].clone()));
}

fn gen_sine_signal(domain: (isize, isize), amplitude: f64, frequency: f64, x_shift: f64, y_shift: f64) -> Option<Vec<(isize, f64)>> {
    if domain.0 > domain.1 {
        return None;
    }

    let signal = (domain.0..=domain.1)
        .map(|x| {
            let sample = amplitude * (frequency * (x as f64 - x_shift)).sin() + y_shift;
            (x, sample)
        })
        .collect();

    return Some(signal);
}

fn gen_cosine_signal(domain: (isize, isize), amplitude: f64, frequency: f64, x_shift: f64, y_shift: f64) -> Option<Vec<(isize, f64)>> {
    if domain.0 > domain.1 {
        return None;
    }

    let signal = (domain.0..=domain.1)
        .map(|x| {
            let sample = amplitude * (frequency * (x as f64 - x_shift)).cos() + y_shift;
            (x, sample)
        })
        .collect();

    return Some(signal);
}

fn discrete_fourier_transform(signal: &[(isize, f64)], domain: (isize, isize)) -> Option<Vec<(isize, f64)>> {
    let mut transformed_signal = Vec::new();
    let n = signal.len();
    let i = Complex::new(0.0, 1.0);

    for k in domain.0..=domain.1 {
        let mut sum = Complex::new(0.0, 0.0);
        for &(x, sample) in signal {
            let exponent = -i * (2.0 * PI * k as f64 * x as f64 / n as f64);
            sum += sample * exponent.exp();
        }
        transformed_signal.push((k, sum.norm()));
    }

    return Some(transformed_signal);
}

