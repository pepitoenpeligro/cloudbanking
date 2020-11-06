## Basic Structure 

```bash
src
├── bankaccount
│   ├── mod.rs
│   ├── model.rs
│   └── test.rs
├── bankcard
│   ├── mod.rs
│   ├── model.rs
│   └── test.rs
├── bankfund
│   ├── mod.rs
│   ├── model.rs
│   └── test.rs
├── controller
│   ├── mod.rs
│   ├── model.rs
│   └── test.rs
├── main.rs
├── paymentgroup
│   ├── mod.rs
│   ├── model.rs
│   └── test.rs
├── savinggroup
│   ├── mod.rs
│   ├── model.rs
│   └── test.rs
├── user
│   ├── mod.rs
│   ├── model.rs
│   └── test.rs
└── utils
    ├── mod.rs
    ├── model.rs
    └── test.rs
```

:surfer: The project can __be compiled__ without errors as indicate the badges: 
![Rust](https://github.com/pepitoenpeligro/CloudBanking/workflows/Rust/badge.svg)
[![Build Status](https://travis-ci.com/pepitoenpeligro/CloudBanking.svg?branch=master)](https://travis-ci.com/pepitoenpeligro/CloudBanking)


```
cargo build
cargo run
```