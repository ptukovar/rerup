# rerup
Command line tool for hacking! 😬☠ <br>
This program is a command-line tool for simulating URL brute-forcing. It utilizes an input file containing a list of lines and systematically combines them with a given URL. For each combination, it performs HTTP GET requests and displays information about the responses, such as the URL, status code, and response size.

## Usage
1. Clone repository `git clone https://github.com/ptukovar/rerup.git`
2. Run by following this format: `cargo run -- -w <file_path> -u <url>/FUZZ` or `cargo run -- -w <file_path> -u FUZZ.<url>`

Example: 
```shell
cargo run -- -w "inputs.txt" -u http://127.0.0.1:8000/FUZZ
```
Result:
```shell
Path: inputs.txt
Url: http://127.0.0.1:8000/FUZZ
-----------------------------------------------------------------
Url: http://127.0.0.1:8000/index.html   Status: 200     Size: "324"
Url: http://127.0.0.1:8000/login        Status: 200     Size: "354"
Url: http://127.0.0.1:8000/admin        Status: 200     Size: "354"
Url: http://127.0.0.1:8000/foofoo       Status: 404     Size: "469"
Url: http://127.0.0.1:8000/booboo       Status: 404     Size: "469"
Url: http://127.0.0.1:8000/support      Status: 404     Size: "469"
Url: http://127.0.0.1:8000/about        Status: 200     Size: "354"
```
