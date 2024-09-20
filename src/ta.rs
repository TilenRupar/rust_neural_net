use ta::indicators::{SimpleMovingAverage, ExponentialMovingAverage};
use ta::Next;

use crate::load_candle::Candle;

pub fn add_ta(candles: Vec<&Candle>) -> Vec<(f64, f64)> {
    fn calculate_sma(candles: Vec<&Candle>, period: usize) -> Vec<f64> {
        let mut sma = SimpleMovingAverage::new(period).unwrap();
        candles.iter().map(|candle| sma.next(candle.close)).collect()
    }
    
    fn calculate_ema(candles: Vec<&Candle>, period: usize) -> Vec<f64> {
        let mut ema = ExponentialMovingAverage::new(period).unwrap();
        candles.iter().map(|candle| ema.next(candle.close)).collect()
    }

    let sma_values = calculate_sma(candles.clone(), 20); // 20-period SMA
    let ema_values = calculate_ema(candles.clone(), 20); // 20-period EMA

    sma_values.into_iter().zip(ema_values.into_iter()).collect()
}