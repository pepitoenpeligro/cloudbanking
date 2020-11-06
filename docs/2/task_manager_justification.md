# Configuration of the task manager and its justification

The task managers __available__ for my project are:

* make
* cargo-make

---


The advantages of `make` are:

* It's **portable**
* It's **native supported** in all unix system (=> cloud is 99% unix)
* Focused in **target** not in methods
* **Cross dependeces** resolvers
* **Easy** to use


An example of using `make` for this project can be found in [commit #b16fe7f](https://github.com/pepitoenpeligro/CloudBanking/blob/master/makefile). Now it's not used, so it was erased. 


---

The advantages of `cargo-make` are:
* It's **portable**
* It's not native but is **supported in all unix** system if you install rust compiler (needed)
* **Based** in **make** tool.
* It's support **Cross Platform Shell**
```toml
[tasks.copyshell]
script_runner = "@shell"
script = [
    '''
    cp -rf target/doc/ docs/documentation" 
    '''
]
```
* It's support **multiple language inline-script**
```toml
[tasks.perl]
script_runner = "perl"
script_extension = "pl"
script = [
    '''
    print "Hi!";
    '''
]

[tasks.javascript]
script_runner = "node"
script_extension = "js"
script = [
    '''
    console.log('Hi!');
    '''
]
```
* It's supports **parallel task**
```toml
tarea = {name = ["A"], parallel = true }
```
* It's supports **inline scripts**
```toml
[tasks.hi]
script = [
    "echo hi...",
    "echo \"Hi again\"",
]
```
* It's support **extern scripts**
```toml
[tasks.hi-externs]
script = { file = "${MY_PATH}/my_script.sh", absolute_path = true }
```
* It's **extendable** by **other makefiles.toml**
```toml
extend = { path = "othermake.toml", optional = true }
```
* It's support **environment variables**
```toml
RUST_BACKTRACE = 1
TEST = "testA"
DEVELOPMENT = false
PRODUCTIO = false
```

An example of using `cargo-make` for this project can be found [make.toml](https://github.com/pepitoenpeligro/CloudBanking/blob/master/make.toml) added in [commit #c99188d](https://github.com/pepitoenpeligro/CloudBanking/commit/c99188d5af9cffe88c0fef246e6d386a9c5e6be3#diff-afa34349f29084381217cb109f9b87b95473960142b7f3d01bde14ef1adfc40b)
 

Our project has the aspiration to be executed in the cloud, so, although the definition of the virtual structure is not completely defined, we are clear that we need a task manager as portable as possible. As `cargo-make` offers the same advantages of `cargo-make` and brings new ones, like the ones described above, we decided to use `cargo-make`.
  

Finally it will be used: **cargo-make**

:warning: The stable version of Rust does NOT allow you to check the syntax of a file. To do this, we would need to update to the *nightly* version and run `rustc --Zparse-only src/*`. In our case, we are not going to use the *nightly* version and we demonstrate that the files are syntactically correct by compiling, so the task in the task manager that demonstrates the syntax is: build and can be launched in the console as `cargo make --makefile make.toml build`


---

You can see the issues related to this study:
1. [Search for Task Managers and evaluate the one that suits me best and justify my choice](https://github.com/pepitoenpeligro/CloudBanking/issues/34)
2. [TaskManager use](https://github.com/pepitoenpeligro/CloudBanking/issues/43)

---


:warning: If you search for _task manager_ in _Google_, you'll find certain _GitHub_ repositories that talk about task managers for Rust that refer to project task managers, not project builders, like _Taskwarrior_. It should be understood that you're not looking for a project task manager (like _Trello_) but a project build manager to automate the necessary actions (very useful in the cloud deployment phase)

## References
* [make](https://www.gnu.org/software/make/manual/make.html)
* [cargo-make](https://github.com/sagiegurari/cargo-make)