// Source for VecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html

use std::collections::VecDeque;

// Boolean: 0 (Highest -> Lowest) 1 (Lowest -> Highest)

pub struct Queue<T: Ord>
{
    vector: VecDeque<T>,
    order: bool, 
}

impl <T: Ord> Queue<T> 
{
    pub fn new(order: bool) -> Self 
    {
        Queue {
            vector: VecDeque::new(),
            order,
        }
    }

    pub fn push(&mut self, dataValue: T) 
    {
        let position = self.vector.iter().position(|x| 
        {    
            if self.order // Lowest -> Highest
            {
                dataValue < *x
            }
            else // Highest -> Lowest
            {
                dataValue > *x
            }
        });

        match position 
        {
            Some(i) => self.vector.insert(i, dataValue),
            None => self.vector.push_back(dataValue),
        }
    }

    pub fn pop(&mut self) -> Option<T>
    {
        if self.order
        {
            return self.vector.pop_back();
        }
        else
        {
            return self.vector.pop_front();
        }
    }
}

impl<T: Ord + std::fmt::Display> Queue<T> // std::fmt::Display had to be added to print inside an Impl as T does not know how to display without it.
{
    pub fn traverse(&mut self)
    {
        for queueItem in self.vector.iter() 
        {
            println!("{}", queueItem);
        }
    }
}
