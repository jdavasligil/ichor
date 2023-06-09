// Ichor - An old school fantasy campaign generator.
// MIT License
// Copyright (c) 2023 Jaedin Davasligil

mod user_input;
use user_input::input;

pub fn interactive_console() {
    let mut line: String; 

    loop {
        line = input("$ ");
        line.make_ascii_uppercase();

        match line.as_str() {
            "Q" | "QUIT" => {
                break;
            },
            "H" | "HELP" => {
                print_help();
            },
            _ => {
                println!("Command does not exist. Type 'h' or 'help' for help.");
            }
        }
    }
}

fn print_help() {
    println!("Help");
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


