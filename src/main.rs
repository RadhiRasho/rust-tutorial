#![allow(unused)]

use rand::Rng;
mod restaurant;
use restaurant::order_food;
use std::cmp::Ordering;
use std::collections::btree_map::Values;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

/* Functions
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

use std::ops::{Add, Div, Mul, Sub};

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
*/

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

fn main() {
    order_food();
}
