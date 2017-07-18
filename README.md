# Rust On The Web

Inspired by [this blog post](https://hoverbear.org/2017/04/06/the-path-to-rust-on-the-web/).  
For more detailed info and background, read it. This is how to setup rust -> wasm/js build on *FreeBSD*.


## System
- FreeBSD 12-CURRENT
- Latest ports installed via `pkg install`

## Install
Install Rust using `rustup`  
`$ curl https://sh.rustup.rs -sSf | sh`  

Add wasm target  
`$ rustup target add wasm32-unknown-emscripten`  

Hopefully soon `pkg install emscripten` will be sufficient but for now we build from source.  
Download, build and install emscripten (takes a while...)  
```
$ curl https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz | tar -xf -  
$ cd ~/emsdk-portable  
$ ./emsdk update  
$ ./emsdk install sdk-incoming-64bit  
$ ./emsdk activate sdk-incoming-64bit  
```

A copy of Linux's `node` is installed. We need to use `node` from ports.  
`$ sudo pkg install node`  

Edit `$HOME/.emscripten` to use `node` from ports (`/usr/local/bin/node`).  

Add to `PATH` (`$DEV` is the folder where I ran the `curl` command):  
`$DEV/emsdk-portable:$DEV/emsdk-portable/clang/fastcomp/build_incoming_64/bin:$DEV/emsdk-portable/emscripten/incoming`  

`$ emcc -v` should now output something like:  
```
emcc (Emscripten gcc/clang-like replacement + linker emulating GNU ld) 1.37.16  
clang version 4.0.0 (https://github.com/kripken/emscripten-fastcomp-clang.git feff46c2a99f012dfdd291f02636fd386d132969) (https://github.com/kripken/emscripten-fastcomp.git d11ecd94546630d1e087de93ac95eccb2630b5e3) (emscripten 1.37.16 : 1.37.16)  
Target: x86_64-unknown-freebsd12.0  
Thread model: posix  
InstalledDir: /usr/home/johannes/dev/emsdk-portable/clang/fastcomp/build_incoming_64/bin  
INFO:root:(Emscripten: Running sanity checks)  
```

## Fun Part

Now to the fun part.  

`cd` back to this project folder.  
Set to use rust nightly.  
`$ rustup override set nightly`  

Build with   
`$ make`   
(this will take a while the first time...)  

Start a webserver:  
`$ python -m SimpleHTTPServer`  

And surf to `http://localhost:8000/site/`  







