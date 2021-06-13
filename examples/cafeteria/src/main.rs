use crossbeam::channel::{self, Receiver, Sender};
use std::{thread, time::Duration};

#[derive(Debug)]
enum Lunch {
    Soup,
    Salad,
    Sandwich,
    HotDog,
}

fn cafeteria_worker(name: &str, orders: Receiver<&str>, lunches: Sender<Lunch>) {
    for order in orders {
        println!("{} receives an order for {}", name, order);
        let lunch = match &order {
            x if x.contains("soup") => Lunch::Soup,
            x if x.contains("salad") => Lunch::Salad,
            x if x.contains("sandwich") => Lunch::Sandwich,
            _ => Lunch::HotDog,
        };
        for _ in 0..order.len() {
            thread::sleep(Duration::from_secs_f32(0.1))
        }
        println!("{} sends a {:?}", name, lunch);
        if lunches.send(lunch).is_err() {
            break;
        }
    }
}

fn main() {
    let (orders_tx, orders_rx) = channel::unbounded();
    let orders_rx2 = orders_rx.clone();
    let (lunches_tx, lunches_rx) = channel::unbounded();
    let lunches_tx2 = lunches_tx.clone();

    let alice_handle = thread::spawn(|| cafeteria_worker("alice", orders_rx2, lunches_tx2));
    let zack_handle = thread::spawn(|| cafeteria_worker("zack", orders_rx, lunches_tx));

    for order in vec![
        "polish dog",
        "caesar salad",
        "onion soup",
        "reuben sandwich",
    ] {
        println!("ORDER: {}", order);
        let _ = orders_tx.send(order);
    }
    drop(orders_tx);

    for lunch in lunches_rx {
        println!("Order Up! -> {:?}", lunch);
    }

    let _ = alice_handle.join();
    let _ = zack_handle.join();
}
