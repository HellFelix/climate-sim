use std::error::Error;

use bevy::app::AppExit;
use bevy::prelude::*;
use plotters::{
    chart::{ChartBuilder, LabelAreaPosition, SeriesLabelPosition},
    prelude::{BitMapBackend, Circle, IntoDrawingArea, Rectangle},
    style::{BLACK, Color, FontStyle, IntoFont, IntoTextStyle, RGBColor, WHITE},
};

use crate::{consts::MAX_TIME, temp::TempMap};

#[derive(Component)]
pub struct TemperatureData {
    southern_sample_temp: Vec<f32>,
    northern_sample_temp: Vec<f32>,
    max_temp: Vec<f32>,
    min_temp: Vec<f32>,
    avg_temp: Vec<f32>,
    time: Vec<f32>,
}

pub fn start_sampling(mut commands: Commands) {
    commands.spawn(TemperatureData {
        southern_sample_temp: vec![],
        northern_sample_temp: vec![],
        max_temp: vec![],
        min_temp: vec![],
        avg_temp: vec![],
        time: vec![],
    });
}

pub fn sample_temp(
    temp_query: Query<&TempMap>,
    mut data_query: Query<&mut TemperatureData>,
    time: Res<Time<Fixed>>,
) {
    let temp = temp_query.single().unwrap();
    let mut data = data_query.single_mut().unwrap();

    let t = time.elapsed_secs();
    data.time.push(t);
    let (max, min, avg, southern_sample, northern_sample) = temp.get_heat_stats();

    // info!("time: {t}, max: {max}, min: {min}, avg: {avg}");

    data.southern_sample_temp.push(southern_sample);
    data.northern_sample_temp.push(northern_sample);
    data.max_temp.push(max);
    data.min_temp.push(min);
    data.avg_temp.push(avg);
}

pub fn plot_data(
    data_query: Query<&TemperatureData>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Virtual>>,
    mut exit_events: ResMut<Events<AppExit>>,
) {
    // info!("Waiting for plot");
    if time.is_paused() && keyboard.just_pressed(KeyCode::KeyC) || time.elapsed_secs() > MAX_TIME {
        info!("Plotting");
        let data = data_query.single().unwrap();

        plot_data_temperature(
            &data.time,
            &data.southern_sample_temp,
            &data.northern_sample_temp,
            &data.max_temp,
            &data.min_temp,
            &data.avg_temp,
            "Time [Ti.U.]",
            "Temperature [Te.U.]",
            "Temperature with regards to time",
            "temperature.png",
        )
        .unwrap();
        exit_events.send(AppExit::Success);
    }
}

fn plot_data_temperature(
    t_vals: &[f32],
    southern_vals: &[f32],
    northern_vals: &[f32],
    max_vals: &[f32],
    min_vals: &[f32],
    avg_vals: &[f32],
    x_name: &str,
    y_name: &str,
    title: &str,
    out_name: &str,
) -> Result<(), Box<dyn Error>> {
    // Set up image and black background
    let root = BitMapBackend::new(out_name, (1000, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // Compute data range
    let x_min = t_vals.iter().cloned().fold(f32::INFINITY, f32::min);
    let x_max = t_vals.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let y1_min = max_vals.iter().cloned().fold(f32::INFINITY, f32::min);
    let y1_max = max_vals.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let y2_min = min_vals.iter().cloned().fold(f32::INFINITY, f32::min);
    let y2_max = min_vals.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let y_min = y1_min.min(y2_min);
    let y_max = y1_max.max(y2_max);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            title,
            ("Calibri", 60, FontStyle::Bold, &BLACK).into_text_style(&root),
        )
        .x_label_area_size(60)
        .y_label_area_size(60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .set_label_area_size(LabelAreaPosition::Left, 100)
        .margin_right(100)
        .build_cartesian_2d((x_min)..(x_max), (y_min)..(y_max))?;

    // Mesh styling
    chart
        .configure_mesh()
        .disable_mesh()
        .axis_style(&BLACK)
        .bold_line_style(&BLACK.mix(0.2))
        .label_style(("monospace", 20).into_font().color(&BLACK))
        .x_desc(x_name)
        .y_desc(y_name)
        .draw()?;

    chart
        .draw_series(
            t_vals
                .iter()
                .zip(southern_vals.iter())
                .map(|(&x, &y)| Circle::new((x, y), 2, RGBColor(255, 0, 255).filled())),
        )?
        .label("Southern Sample")
        .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], RGBColor(255, 0, 255)));

    chart
        .draw_series(
            t_vals
                .iter()
                .zip(northern_vals.iter())
                .map(|(&x, &y)| Circle::new((x, y), 2, RGBColor(255, 255, 0).filled())),
        )?
        .label("Northern Sample")
        .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], RGBColor(255, 255, 0)));

    chart
        .draw_series(
            t_vals
                .iter()
                .zip(avg_vals.iter())
                .map(|(&x, &y)| Circle::new((x, y), 2, RGBColor(0, 255, 0).filled())),
        )?
        .label("Average Temperature")
        .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], RGBColor(0, 255, 0)));

    chart
        .draw_series(
            t_vals
                .iter()
                .zip(max_vals.iter())
                .map(|(&x, &y)| Circle::new((x, y), 2, RGBColor(255, 0, 0).filled())),
        )?
        .label("Max Temperature")
        .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], RGBColor(255, 0, 0)));

    chart
        .draw_series(
            t_vals
                .iter()
                .zip(min_vals.iter())
                .map(|(&x, &y)| Circle::new((x, y), 2, RGBColor(0, 0, 255).filled())),
        )?
        .label("Min Temperature")
        .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], RGBColor(0, 0, 255)));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::MiddleRight)
        .margin(20)
        .legend_area_size(5)
        .border_style(BLACK)
        .background_style(WHITE.mix(0.1))
        .label_font(("Calibri", 20))
        .draw()?;

    println!("Plot saved to {out_name}");
    Ok(())
}
