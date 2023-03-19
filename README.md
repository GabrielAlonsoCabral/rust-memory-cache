# rust-memory-cache
  Rust Memory Cache is a TCP simple single thread memory cache developed in Rust using RESP Protocol.
  
  Developed by: <a href="https://www.github.com/gabrielAlonsoCabral">@GabrielAlonsoCabral</a>  
 <br/>

## Installation

```
# clone this repository
$ git clone https://github.com/GabrielAlonsoCabral/rust-memory-cache.git
$ cd rust-memory-cache
```

<br/>

## Start Server

```
$ cargo run
```

## Build

```
$ cargo build --release
```

## Usage

```
# Test connection
$ echo "+ping\r\n" | nc 127.0.0.1 6379

# Storing a value in memory
$ echo "+set user {id:1} 10\r\n" | nc 127.0.0.1 6379

# Getting a value in memory
$ echo "+get user\r\n" | nc 127.0.0.1 6379
```
