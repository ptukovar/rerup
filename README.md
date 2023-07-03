## rerup
Command line tool for hacking! 😬☠
This program is a command-line tool for simulating URL brute-forcing. It utilizes an input file containing a list of lines and systematically combines them with a given URL. For each combination, it performs HTTP GET requests and displays information about the responses, such as the URL, status code, and response size.

## Usage
1. Clone repository `git clone https://github.com/ptukovar/rerup.git`
2. Run by following this format: `cargo run -- -w <file_path> -u <url>`

   Example: 
   ```shell
   cargo run -- -w "inputs.txt" -u http://127.0.0.1:8000/
