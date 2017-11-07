# Niceprofit

![Travis CI Build status](https://travis-ci.org/StefanoChiodino/niceprofit.svg?branch=master)
![Appveyor Build status](https://ci.appveyor.com/api/projects/status/bt5b5t25x4wi8ttk/branch/master?svg=true)

## TL;DR

Download latest for your OS [In the release page](https://github.com/StefanoChiodino/niceprofit/releases).

Extract (`tar -xf`)

Run `niceprofit -p INSER_PATH_TO_CPUMINER_HERE`

### OR
Install Rust, clone, compile and run.

```
curl https://sh.rustup.rs -sSf | sh
git clone git@github.com:StefanoChiodino/niceprofit.git
cd niceprofit
cargo run --bin niceprofit -- -p INSER_PATH_TO_CPUMINER_HERE
```

## Intro

Nicehash Windows Gui is very good and it takes care of everything. It benchmarks your CPU/GPU and then keep switching to the most profitable algorithm.

On linux there is [not such nice option](https://www.nicehash.com/help/does-nicehash-miner-work-on-windows-xp-or-linux). [Nicehash advises to use](https://www.nicehash.com/help/cpu-mining) [tpruvot's cpuminer](https://github.com/tpruvot/cpuminer-multi) but there is no autoswitching, benchmarking, etc build on top of it.

Niceprofit goal is to find your CPU hashrates for all algorithms and then select the most profitable with the up to date data from Nicehash API.

## Usage

### Install Rust

[Follow the official website to install rust](https://www.rust-lang.org/en-US/install.html).

This is usually done via rustup as simply as running `curl https://sh.rustup.rs -sSf | sh`

### Compile and run Niceprofit

Follow [Nicehash instructions on how to install cpuminer](https://www.nicehash.com/help/cpu-mining). Then either put the executable in the bin folder, add the path to the executable to $PATH, or specify the path in the command line of Niceprofit

`cargo run --bin niceprofit` executed inside Niceprofit's folder will compile it and run it.
Use `cargo run --bin niceprofit -- --help` to see the options.
```
OPTIONS:
    -p, --cpuminer-path <cpuminer_path>
            Sets the path of cpuminer [default: cpuminer]
```

Niceprofit will benchmark all available algorithms for 100 seconds by default.

## Roadmap

For now Niceprofit only benchmarks all the algorithms and tell you their profitability based on your CPU, and which one is the most profitable.

I hope to make Niceprofit as good as the Windows GUI alternative, switching between the most profitable algorithm in real time, and maybe adding GPU support.
