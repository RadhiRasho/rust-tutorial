#![allow(unused)]

use rand::Rng;
mod restaurant;
use restaurant::order_food;
use std::cmp::Ordering;
use std::collections::btree_map::Values;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::{Add, Div, Mul, Sub};

fn get_sum_2(x: i32, y: i32) -> i32 {
    return x + y;
}
fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}
fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }

    return sum;
}
fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}
fn get_sub_gen<T: Sub<Output = T>>(x: T, y: T) -> T {
    return x - y;
}
fn get_mul_gen<T: Mul<Output = T>>(x: T, y: T) -> T {
    return x * y;
}
fn get_div_gen<T: Div<Output = T>>(x: T, y: T) -> T {
    return x / y;
}

fn trait_stuff() {
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32,
    };
    struct Circle {
        length: f32,
        width: f32,
    };

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);

    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rec Area: {}", rec.area());
    println!("circ Area: {}", circ.area());
}

fn order() {
    order_food();
}

// File Creation, Error Handling
fn file_stuff() {
    let path = "lines.txt";

    let output = File::create(path);

    let mut output = match output {
        Ok(x) => x,
        Err(err) => panic!("Problem creating file : {:?}", err),
    };

    write!(output, "Just some \nRandom Words").expect("Failed to write to the file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("random.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Can't Create File : {:?}", error),
            },
            _other_error => panic!("Proglem Opening File : {:?}", error),
        },
    };
}

fn iterator_stuff() {
    let mut arr_it = [1, 2, 3, 4];

    for i in arr_it.iter() {
        println!("{}", i);
    }

    let mut iter1 = arr_it.iter();

    println!("1st : {:?}", iter1.next())
}

// Closures
// let var_name = |params| -> return_type { Body }
fn closures_round_one() {
    let can_vote = |age: i32| {
        return age >= 18;
    };

    println!("Can Vote: {} ", can_vote(8));
}

fn closures_round_two() {
    let mut sampl1 = 5;

    let print_var = || println!("sampl1 = {}", sampl1);
    print_var();

    sampl1 = 10;

    let mut change_var = || sampl1 += 1;

    change_var();
    println!("sampl1 = {}", sampl1);
    sampl1 = 10;
    println!("sampl1 = {}", sampl1)
}

fn closures_round_three() {
    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        return func(a, b);
    }

    let sum = |a, b| a + b;
    let prod = |a, b| a * b;

    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));
}

// Smart Pointers
// & -> Box
// * ->

fn main() {}
