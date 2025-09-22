# OxM8

OxM8 is a chess engine written in Rust. It aims to be an efficient and simple engine that can be used for analysis and playing games. Implementation includes full game rules, a basic evaluation function, and a search algorithm.

## Table of Contents

<!-- TOC -->
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
    - [Rust](#rust)
    - [Cargo](#cargo)
  - [Building the Project](#building-the-project)
  - [Running the Engine](#running-the-engine)
- [Testing](#testing)
<!-- /TOC -->

## Features

- Full chess rules implementation
- Basic evaluation function
- Minimax search with alpha-beta pruning
- FEN support
- ... (more to come!)

## Getting Started

### Prerequisites

#### Rust

Make sure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).

#### Cargo

Cargo is the Rust package manager and comes bundled with Rust. You can check if you have it installed by running:

```bash
cargo --version
```

### Building the Project

Clone the repository:

```bash
git clone https://github.com/malcolm-a/OxM8.git
cd OxM8
```

Build the project using Cargo:

```bash
cargo build
```

### Running the Engine
You can run the engine using Cargo:

```bash
cargo run
```

This will start the engine in a command-line interface. You will be prompted to chose between playing on a board or to analyze a position using FEN notation. More to come soon!

## Testing

Run the tests using Cargo:

```bash
cargo test
```
