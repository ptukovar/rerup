# rerup
Command line tool for hacking! 😬☠ <br>
--
This program is a command-line tool for simulating URL brute-forcing. It utilizes an input file containing a list of lines and systematically combines them with a given URL. For each combination, it performs HTTP GET requests and displays information about the responses, such as the URL, status code, and response size. The results will be save in file `output.txt`.

## Help
1. For help use `cargo run -- -h` or `cargo run -- -help`<br>
Result:
```shell
Usage: -w <path> -u <url> -o <output_file>
Options:
-h, -help       Display this help message [--]
-w              Specify the input file path [--]
-u              Specify the URL with 'FUZZ' as a placeholder
-o              Specify the output file path
-st             Filter by status code (e.g., -st =200)
-si             Filter by response size (e.g., -si >1000)
Example: rerup -w inputs.txt -u http://127.0.0.1:8000/FUZZ -o output.txt -st =200
```

## Usage
1. Clone repository `git clone https://github.com/ptukovar/rerup.git`
2. Run by following this format: `cargo run -- -w <file_path> -u <url>/FUZZ` or `cargo run -- -w <file_path> -u FUZZ.<url>` <br>

Example: 
```shell
cargo run -- -w "inputs.txt" -u http://127.0.0.1:8000/FUZZ -o "output.txt"
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
3. Additionally, you can use the filters for Status `-st` or Size `-si` with the following parameters: `=` ,  `!=` ,  `<` ,  or `>`

Example: 
```shell
cargo run -- -w "inputs.txt" -u http://127.0.0.1:8000/FUZZ -o "output.txt" -si "<350"
```

For educational purposes only!
