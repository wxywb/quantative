#[derive(Debug, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub enum OrderType {
    Limit,
    Market,
    IOC,
    FOK,
}

#[derive(Debug, Clone)]
pub enum TimeInForce {
    GTC, // Good Till Canceled
    IOC, // Immediate Or Cancel
    FOK, // Fill Or Kill
}

#[derive(Debug, Clone)]
pub struct OrderRequest {
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub gateway: Option<String>,
    pub time_in_force: Option<TimeInForce>,
    pub stop_price: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct CancelRequest {
    pub symbol: Option<String>,

    pub client_order_id: Option<String>,

    pub order_id: String,

    pub gateway: Option<String>,
}
