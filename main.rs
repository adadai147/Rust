// Authored by 李凌瑶 in 2024/06/21

use rand::Rng; // 引入随机数生成器
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
enum Side { // 定义了两种交易方向：买入（BID）和卖出（ASK）
    BID,
    ASK,
}

#[derive(Debug, Clone, Copy)]
struct Trade {
    price: f32,
    quantity: f32,
    side: Side,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    avg_price: f32,
    quantity: f32,
    side: Side,
}

fn calc(pos: Position, trades: Vec<Trade>) -> (Position, f32) {
    let mut position = pos;
    let mut realized_pnl = 0.0;

    for trade in trades {
        let (new_avg_price, new_quantity, trade_pnl) = match (position.side, trade.side) {
            (Side::BID, Side::BID) | (Side::ASK, Side::ASK) => {
                // 如果持仓和交易同向，合并持仓。计算新的平均价格new_avg_price、新的持仓数量new_quantity（当前持仓数量加上交易数量）。
                let new_avg_price = (position.avg_price * position.quantity + trade.price * trade.quantity) / (position.quantity + trade.quantity);
                (new_avg_price, position.quantity + trade.quantity, 0.0)
            },

            (Side::BID, Side::ASK) => {
                // 计算盈亏(pnl)：如果交易数量小于或等于持仓数量，则按交易数量计算盈亏；如果交易数量大于持仓数量，则按持仓数量计算盈亏。
                let pnl = (trade.price - position.avg_price) * position.quantity.min(trade.quantity);
                // 计算新的平均价格和持仓数量
                let (new_avg_price, new_quantity,new_side) = if position.quantity > trade.quantity {
                    (position.avg_price, position.quantity - trade.quantity, position.side)
                } else {
                    (trade.price, trade.quantity-position.quantity, trade.side)
                };
                position.side = new_side;
                (new_avg_price, new_quantity, pnl)
            },
            (Side::ASK, Side::BID) => {
                // 同理
                let pnl = (position.avg_price-trade.price) * position.quantity.min(trade.quantity);
                let (new_avg_price, new_quantity,new_side) = if position.quantity > trade.quantity {
                    (position.avg_price, position.quantity - trade.quantity,position.side)
                } else {
                    (trade.price, trade.quantity-position.quantity,trade.side)
                };
                position.side = new_side;
                (new_avg_price, new_quantity, pnl)
            },
        };

        position.avg_price = new_avg_price;
        position.quantity = new_quantity;
        realized_pnl += trade_pnl;
    }

    (position, realized_pnl)
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut trades: Vec<Trade> = Vec::new();

    // 生成10000个随机成交数据
    for _ in 0..10000 {
        let price: f32 = rng.gen_range(0.0..1000.0);
        let quantity: f32 = rng.gen_range(1.0..100.0);
        let side = if rng.gen() { Side::BID } else { Side::ASK };
        trades.push(Trade { price, quantity, side });
    }

    let init_pos = Position {
        avg_price: 100.0,
        quantity: 10.0,
        side: Side::BID,
    };

    let start_time = Instant::now();
    let (updated_pos, pnl) = calc(init_pos, trades);
    let duration = start_time.elapsed();

    println!("Updated Position: {:?}", updated_pos);
    println!("Realized PnL: {}", pnl);
    println!("Total time taken: {:?}", duration);

    // 计算平均运行时间
    let average_time_per_trade = duration / 10000;
    println!("Average time per trade: {:?}", average_time_per_trade);
}