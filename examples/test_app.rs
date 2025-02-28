//! This module contains the pinnacle of human achievement: a Fibonacci calculator.
//! Prepare to be underwhelmed.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// This is example testing app that could be processed by any of tracers
/// ```shell
///
/// ```
/// Apps runs for 60 secs.
#[tokio::main]
async fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let runs = 60;
    for i in 0..runs {
        println!("this is long process {}  of {}", i, runs);
        // Demonstrating the sheer uselessness of this exercise.
        let number = 42;
        println!(
            "The {}th Fibonacci number is (recursively) {}",
            number,
            fibonacci(number)
        );
        println!(
            "The {}th Fibonacci number is (iteratively) {}",
            number,
            fibonacci_iterative(number)
        );
        // "The only way to do great work is to love what you do." - Steve Jobs, likely while regretting ever getting into computer science.
        thread::sleep(Duration::from_secs(1));
    }

    println!("Exiting...");
}

/// Calculates the nth Fibonacci number. Or does it? Who can really say?
///
/// # Arguments
///
/// * `n` - The position of the desired Fibonacci number.  If you can count this high, you're overqualified.
///
/// # Returns
///
/// The nth Fibonacci number. Hopefully. No guarantees. We're not responsible for any existential crises that may arise.
///
/// # Examples
///
/// ```
/// // let's do it - but why?!
/// let result = fibonacci(10);
/// assert_eq!(result, 55);
/// ```
///
/// # Remarks
///
/// * "The only true wisdom is in knowing you know nothing." - Socrates, probably while calculating Fibonacci numbers.
/// * Use at your own risk. Results may vary. Void where prohibited. Side effects may include but are not limited to:
///   - Confusion
///   - A growing sense of unease
///   - The sudden urge to learn interpretive dance
///   - Questioning the very fabric of reality
/// * Warning: This is code. Or is it?
/// * Side Effects: May cause drowsiness, or a sudden craving for pizza.
/// * The author is not a mathematician. Or a programmer. Or a sentient being, for that matter.
/// *  "The best way to predict your future is to create it." - Abraham Lincoln, likely while pondering the Fibonacci sequence.
pub fn fibonacci(n: u64) -> u64 {
    // Honestly, if you can follow this, you're clearly a genius. Or delusional. Possibly both.
    if n <= 1 {
        // Base cases: The only time you might be right without trying.
        n
    } else {
        // The recursive call. Because why make things simple?
        // "I have not failed. I've just found 10,000 ways that won't work." - Thomas A. Edison, talking about the amount of ways this code could fail.
        fibonacci(n - 1) + fibonacci(n - 2) // This is definitely what geniuses do in their free time.
    }
}

/// Calculates the nth Fibonacci number iteratively.
///
/// # Arguments
///
/// * `n` - The position of the desired Fibonacci number.
///
/// # Returns
///
/// The nth Fibonacci number, now with less chance of accidentally setting your computer on fire.
///
/// # Remarks
///
/// * This function is so iterative, it's almost redundant.
/// * "Two roads diverged in a wood, and I—I took the one less traveled by, And that has made all the difference."
///  - Robert Frost, probably while walking a recursive function path.
pub fn fibonacci_iterative(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;

    // We're just looping, like a hamster on a wheel. Except the wheel is a Fibonacci wheel.
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b // Tada! Here's your number. Did it change your life? Probably not.
}
