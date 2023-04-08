// Ichor - An old school fantasy campaign generator.
// MIT License
// Copyright (c) 2023 Jaedin Davasligil

use std::env;

// The application can run in one of two modes:
// Auto - Automatically generates everything;
// Interactive - Runs as a console application.
enum AppMode {
    Auto,
    Interactive,
}

// Arguments:
// 1. Mode - i or blank = Interactive, a = Auto.
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mode: AppMode = AppMode::Auto;

    match args.len() {
        // No arguments are passed.
        1 => {
            println!("No args.");
        },
        // One or more arguments are passed.
        _ => {
            if args[1].eq_ignore_ascii_case("i") {
                mode = AppMode::Interactive;
            }
            println!("Mode is passed.");
        }
    }
    match mode {
        AppMode::Auto => {
        }
        AppMode::Interactive => {
        }
    }
}


// MIT License

// Copyright (c) 2023 Jaedin Davasligil

//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.


