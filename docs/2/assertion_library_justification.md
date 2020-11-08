# Choice and justification of the assertion library used

The __assertion libraries__ for my project are:

* Assertion Macros from Rust
* Galvanic-Assert


---


The advantages of **Assertion Macros from Rust** are:

* It's native 
* It's simple

An example of using the native `Assertion Macros from Rust` for this project can be found in each *submodule/test/test* for example: [test.rs](https://github.com/pepitoenpeligro/CloudBanking/blob/master/src/bankaccount/test.rs) added in [commit #57aaec1](https://github.com/pepitoenpeligro/CloudBanking/commit/3c99ae17ad243a9c31496c395886cb273caf155c#diff-25ffd7e3123f833506863ad8729b4cdf543d74c9a067f06bc788d7c8bfa6b245)


---


The advantages of **Galvanic** are:


* It's **natural description friendly**:

```rust
// Check id is equal
assert_that!(acc.get_id(), eq(String::from("507f1f77bcf86cd799439011")));
// Check vector contains
assert_that!(user.get_bank_accounts(), contains_in_any_order(acc));
// Check value of balance between 0 and 100000
assert_that!(&acc.get_balance(), any_of!(greater_than(0), less_than(100000)));
```
* It's avoid the use of **if-let for DataType Checks**:

```rust
FROM:

if let Account::a = SavingAccount{
    println("a is account{} ",a)
}

TO:

assert_that!(&a, is_variant!(Account));

```
* It's **support error messages**:

```rust
assert_that!(user.get_bank_accounts().len != 0, otherwise "user should have at least one bank account");

```

---


Taking into account all the above, it is clear that we seek maximum ease in all phases of operations of a **DevOps**, so anything you bring and do not subtract, will be welcome. In our case, the library **Galvanic-Assert** provides a syntax more familiar with natural language and functions that facilitate very common tasks in the test (avoiding wasting time implementing myself functions already created and tested).

An example of using `Galvanic-Assert` for this project can be found in each *submodule/test/test_galvanic* for example: [test_galvanic.rs](https://github.com/pepitoenpeligro/CloudBanking/blob/master/src/bankaccount/test_galvanic.rs) added in [commit #a458156](https://github.com/pepitoenpeligro/CloudBanking/commit/a458156853525f0f6a7af5d801cf9c904b71b306)


---

You can see the issues related to this study:
1. [Study Test Libraries and Justify Election](https://github.com/pepitoenpeligro/CloudBanking/issues/38)

---

## References
* [Assertion Macros](https://svartalf.info/posts/2020-03-13-assertion-macros-for-rust/)
* [Galvanic Assert](https://github.com/mindsbackyard/galvanic-assert)