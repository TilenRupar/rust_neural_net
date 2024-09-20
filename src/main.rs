
mod load_candle;
mod plot_candles;
mod ta;
use ta::add_ta;
use plot_candles::plot_candles;
use load_candle::{initialize, get_candle_vector, fetch_all_candles};


fn find_local_extrema(vec: &Vec<f64>, window_size: usize, minima: bool) -> Vec<usize> {
    let mut local_extrema: Vec<usize> = Vec::new();
    
    for i in 0..vec.len() {
        // Determine the window boundaries
        let start = if i >= window_size { i - window_size } else { 0 };
        let end = std::cmp::min(i + window_size + 1, vec.len());

        let current_val = vec[i];

        let is_local_extrema = if minima {
            // Check for local minima
            vec[start..end].iter().all(|&x| x >= current_val)
        } else {
            // Check for local maxima
            vec[start..end].iter().all(|&x| x <= current_val)
        };
        if is_local_extrema {
            local_extrema.push(i);
        }
    }
    local_extrema
}

#[tokio::main]
async fn main() {
    let _ = initialize().await;
    let pair = "SOLBNB";
    let candles = fetch_all_candles(pair.to_string())
    .await
    .unwrap();
    let _ta_results: Vec<(f64, f64)> = add_ta(candles.iter().collect());
    let one_percent_of_candles = (candles.len() as f64 * 0.1).floor() as usize;
    // for window in candles.windows(one_percent_of_candles) {
        
    let ts_close = get_candle_vector(&candles, |candle| candle.close);
    println!("One percent of candles: {}",one_percent_of_candles);
    println!("TS close :{}", ts_close.len());
    let peaks: Vec<usize> = find_local_extrema(&ts_close, one_percent_of_candles, false); 
    let valleys: Vec<usize> = find_local_extrema(&ts_close, one_percent_of_candles, true); 
    let _ = plot_candles(&candles);
    println!("Local maxima: {:?}", peaks);
    println!("Local minima: {:?}", valleys);
}
    