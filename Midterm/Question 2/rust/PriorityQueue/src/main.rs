use PriorityQueue::Queue;
use std::io;

fn main() {
    println!("How would you like your queue?");
    println!("0: Highest -> Lowest Priority");
    println!("1: Lowest -> Highest Priority");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let input = input.trim();

    let mut queue = match input {
        "0" => Queue::new(false),
        "1" => Queue::new(true),
        _   => {
            println!("Invalid input, defaulting to false. Highest -> Lowest");
            Queue::new(false)
        }
    };

    println!("-----------------------");
    queue.push(5);
    queue.push(12);
    queue.push(1);
    queue.push(15);
    queue.push(6);
    queue.push(7);
    queue.traverse();
    println!("-----------------------");
    println!("Popping!");
    println!("-----------------------");
    queue.pop();
    queue.traverse();
    println!("-----------------------");
}
