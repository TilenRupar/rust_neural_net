use plotters::prelude::*;
// use chrono::{DateTime, Utc};
use crate::load_candle::{get_candle_vector, Candle};
use std::fs::OpenOptions;
use std::io::ErrorKind;
// use std::ops::Range;
use std::string::String;

fn check_path_writable(path: &str) -> Result<(), String> {

    match OpenOptions::new().write(true).create(true).open(path) {
        Ok(file) => {
            let metadata = file.metadata().map_err(|err| err.to_string())?;
            if metadata.is_file() && metadata.permissions().readonly() {
                Err("File is read-only.".to_string())
            } else {
                Ok(())
            }
        }
        Err(err) => {
            if err.kind() == ErrorKind::PermissionDenied {
                Err("Permission denied.".to_string())
            } else {
                Err(format!("Error opening file: {}", err))
            }
        }
    }
}

pub fn plot_candles(candles: &Vec<Candle>) ->  Result<(), String> {
    const OUT_FILE_NAME: &str = "C:/rust/rust_neural_net/model_plots/close_price_history.png";
    match check_path_writable(OUT_FILE_NAME) {
        Ok(_) => println!("File is writable."),
        Err(err) => println!("Error: {}", err),
    }
    // let data = get_data();
    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)
        .map_err(|err| format!("Error building chart: {}", err))?;

    let close_prices: Vec<f64> = get_candle_vector(&candles, |candle| candle.close);
    let (from_date,
        to_date,
        min_close_price,
        max_close_price
        ) = 
        (candles.first().unwrap().open_time, 
        candles.last().unwrap().open_time, 
        close_prices.iter().min_by(|a, b| a.partial_cmp(b).expect("No")).unwrap(),
        close_prices.iter().max_by(|a, b| a.partial_cmp(b).expect("No")).unwrap()
        );

    let y_range = *min_close_price as f32..*max_close_price as f32;
    
    println!("y_range: {:?}", from_date..to_date);
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("close price", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(from_date..to_date, y_range)
        .map_err(|err| format!("Error building chart: {}", err))?;

    chart
        .configure_mesh()
        .light_line_style(WHITE)
        .draw()        
        .map_err(|err| format!("Error building chart: {}", err))?;

    chart
        .draw_series(
        candles
            .iter()
            .map(|x| {
            CandleStick::new(
                x.open_time, 
                x.open as f32, 
                x.high  as f32, 
                x.low as f32, 
                x.close as f32, 
                GREEN.filled(), 
                RED, 
                15)
        }),
    ).map_err(|err| format!("Error building chart: {}", err))?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    // root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}
  


    
    