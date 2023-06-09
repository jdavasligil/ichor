// Ichor - An old school fantasy campaign generator.
// MIT License
// Copyright (c) 2023 Jaedin Davasligil

mod console;
use console::interactive_console;
use std::env;

// The application can run in one of two modes:
// Auto - Automatically generates everything;
// Interactive - Runs as a console application.
enum AppMode {
    Auto,
    Interactive,
}

const DEFAULT_MODE: AppMode = AppMode::Interactive;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mode: AppMode = DEFAULT_MODE; 

    match args.len() {
        // No arguments are passed.
        1 => {
            // Do nothing.
        },
        // One or more arguments are passed.
        _ => {
            if args[1].eq_ignore_ascii_case("i") {
                mode = AppMode::Interactive;
            }
            else if args[1].eq_ignore_ascii_case("a") {
                mode = AppMode::Auto;
            }
        }
    }

    match mode {
        AppMode::Auto => {
            println!("Auto is a WIP!");
        }
        AppMode::Interactive => {
            interactive_console();
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


