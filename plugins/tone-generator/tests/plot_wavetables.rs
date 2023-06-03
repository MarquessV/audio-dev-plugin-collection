//! Test that plots wavetables for manual verification.

use plotters::prelude::*;

use tone_generator::oscillator::wavetable::{self, TABLE_SIZE};

const OUT_FILE_NAME: &str = "wavetables.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wavetables = [
        ("Sine", &RED, wavetable::get_sin_table()),
        ("Triangle", &BLUE, wavetable::get_triangle_table()),
        ("Saw", &GREEN, wavetable::get_saw_table()),
        ("Square", &YELLOW, wavetable::get_square_table()),
        ("Noise", &MAGENTA, wavetable::get_noise_table()),
    ];

    let path = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(OUT_FILE_NAME);
    let root_area = BitMapBackend::new(&path, (1920, 1080)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let splits = root_area.split_evenly((wavetables.len(), 1));

    let x_axis = (0..TABLE_SIZE).step(1);
    let y_axis = (-1.0f32..1.0f32).step(0.001);

    for (split, (caption_label, color, wavetable)) in splits.iter().zip(wavetables.iter()) {
        let mut cc = ChartBuilder::on(split)
            .margin(20)
            .set_all_label_area_size(50)
            .caption(caption_label, ("sans-serif", 40))
            .build_cartesian_2d(x_axis.clone(), y_axis.clone())?;

        cc.configure_mesh()
            .x_labels(20)
            .x_desc("sample index")
            .y_labels(10)
            .y_desc("amplitude")
            .x_label_formatter(&|v| format!("{:}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .draw()?;

        cc.draw_series(LineSeries::new(
            x_axis.values().map(|x| (x, wavetable[x])),
            color,
        ))?;

        cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, 0.0)), BLACK))?;

        cc.draw_series(LineSeries::new(
            y_axis.values().map(|y| (TABLE_SIZE / 2, y)),
            BLACK,
        ))?;

        cc.configure_series_labels().border_style(BLACK).draw()?;
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root_area.present().expect("Unable to write result to file");
    println!("Result has been saved to {}", path.to_string_lossy());
    Ok(())
}

#[test]
fn entry_point() {
    main().unwrap();
}
