use std::f64::consts::PI;

const SAMPLE_SIZE: usize = 100;

fn main() {
    let composition_signal = fourier_transform(400.0, SAMPLE_SIZE, 10.0)
        .expect("Could not perform Fourier transform");

    println!("Frequency Comp:  {:?}", composition_signal);
}

fn fourier_transform(frequency: f64, sample_size: usize, scaling_factor: f64) -> Result<Vec<(usize, f64)>, ()> {
    let mut signal_composition_wave: Vec<(usize, f64)> = Vec::new();

    for i in 0..=sample_size {
        let a = (scaling_factor * (i as f64 + frequency)).sin();
        let c = (scaling_factor * (i as f64 - frequency)).sin();

        let b = (2.0 * PI).sqrt() * (sample_size as f64 + frequency);
        let d = (2.0 * PI).sqrt() * (sample_size as f64 - frequency);

        let sample = (a / b) + (c / d);
        signal_composition_wave.push((i, sample));
    }

    return Ok(signal_composition_wave);
}

