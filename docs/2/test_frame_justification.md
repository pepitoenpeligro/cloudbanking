# Choice and justification of the test frame used

The test framework __availables__ for my project are:

* Core Cargo Test
* Stainless
  


----


**Core Cargo Test**

👍 : It's native from Rust
👍 : It's easy to use
```bash
cargo test <moduleToTest>
```
```bash
cargo test
```
👍 : It's really fast and indicate time dedicated in each target (benchmark of test native).
👍 : It allows standard output messages
```cargo test -- --nocapture ```


---


**Stainless**



👍 : It allows you to quickly *generate complex testing hierarchies*

👎 : requires the *nightly* version of the Rust compiler!
👎 : is *not possible* to put *use statements inside the describe! blocks*. (Bad practice)
```rust
#[cfg(test)]
mod accounts {
    pub use std::collections::HashMap;

    describe! stainless {
        it "can create accounts HashMap" {
            let accounts : HashMap<String,Account> = HashMap::new();
        }
    }
}
```

![Core Cargo Test](https://github.com/pepitoenpeligro/CC-Project/blob/master/docs/img/2/test.png)


An example of using the native `Core Cargo Test` for this project can be found in each *submodule/test/test* for example: [test.rs](https://github.com/pepitoenpeligro/CloudBanking/blob/master/src/bankaccount/test.rs) added in [commit #57aaec1](https://github.com/pepitoenpeligro/CloudBanking/commit/3c99ae17ad243a9c31496c395886cb273caf155c#diff-25ffd7e3123f833506863ad8729b4cdf543d74c9a067f06bc788d7c8bfa6b245)

Finally it will be used: **Core Cargo Test**


---


You can see the issues related to this study:
1. [Search and choose the test framework that best suits the project and test based development](https://github.com/pepitoenpeligro/CloudBanking/issues/35)


---

## References
* [Cargo Test](https://doc.rust-lang.org/stable/rust-by-example/cargo/test.html)
* [Stainless](https://github.com/reem/stainless)