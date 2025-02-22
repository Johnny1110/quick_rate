use serde::Deserialize;
use colored::*;

#[derive(Debug, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub last: f64,
    pub lowestAsk: f64,
    pub highestBid: f64,
    pub percentageChange: f64,
    pub volume: f64,
    pub high24Hr: f64,
    pub low24Hr: f64,
    pub base: String,
    pub quote: String,
    pub active: bool,
    pub size: f64,
    pub minValidPrice: f64,
    pub minPriceIncrement: f64,
    pub minOrderSize: f64,
    pub maxOrderSize: f64,
    pub minSizeIncrement: f64,
    pub openInterest: f64,
    pub openInterestUSD: f64,
    pub contractStart: i64,
    pub contractEnd: i64,
    pub timeBasedContract: bool,
    pub openTime: i64,
    pub closeTime: i64,
    pub startMatching: i64,
    pub inactiveTime: i64,
    pub fundingRate: f64,
    pub contractSize: f64,
    pub maxPosition: f64,
    pub minRiskLimit: f64,
    pub maxRiskLimit: f64,
    pub availableSettlement: Option<String>,
    pub futures: bool,
    pub isMarketOpenToOtc: bool,
    pub isMarketOpenToSpot: bool,
}

impl MarketData {
    pub(crate) fn display(&self) {
        println!("---------------------------------------");
        // Symbol printed in yellow
        println!("Symbol: {}", self.symbol.yellow());

        // Price colored based on percentage change:
        // Red if negative, Green if positive
        let price_str = if self.percentageChange < 0.0 {
            format!("{}", self.last).red()
        } else {
            format!("{}", self.last).green()
        };
        println!("Price: {}", price_str);

        println!("High 24Hr: {}", self.high24Hr);
        println!("Low 24Hr: {}", self.low24Hr);

        // Percentage change colored as above
        let percentage_change_str = if self.percentageChange < 0.0 {
            format!("{}", self.percentageChange).red()
        } else {
            format!("{}", self.percentageChange).green()
        };
        println!("Percentage Change: {}", percentage_change_str);
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct FundingRate {
    pub rate: f64,
}

impl FundingRate {
    pub(crate) fn display(&self) {
        let percentage_change_str = if self.rate < 0.0 {
            format!("{}", self.rate).red()
        } else {
            format!("{}", self.rate).green()
        };
        println!("funding fee: {}", percentage_change_str);
    }
}