# Feasibility Study

With the proposed technologies, I want to study the feasibility of the project. I do not want to use unstable technologies or those that generate problems in the long run. Neither do I want technologies that are incompatible with each other.

To solve this problem, I have created a minimal microservice following the __API REST__ pattern in Rust, which can be found at ![music_store](https://github.com/pepitoenpeligro/music_store)


## Language

I have chosen Rust. It is efficient, generates code without dependencies, supports concurrency, is typed (it forces you to think about the type of background), supports multiple paradigms (object oriented, event oriented) and is similar to C++.

I have valued the alternative Scala, but in front of Rust it loses in:

* It uses the virtual machine of java. It is excessively slow for minimal projects. I can have problems in the future with deployments and budgets.
* The configuration of libraries is a chaos. Incompatible versions between them.
* As a developer it forces you to functional programming although it offers the possibility of object oriented programming.

The feasibility study of the project written in Scala can be consulted in a small project located at [MusicStore](https://github.com/pepitoenpeligro/MusicStore)


### Frameworks and Rust libraries 

In this section I explain which frameworks and libraries I use and where.

1. __warp__. It is a framework to create a web server, which allows parameter extraction in paths, path answering, header parameter extraction, string serialization and deserialization, websockets, JSON bodies, Gzip compression, asynchronous communication, HTTP1 and HTTP2. It is used within `main.rs` and works.
2. __serde__. It is a framework to serialize and deserialize data structures in an efficient and generalized way. It allows to work with JSON, YAML, TOML, BSON, URL, and a lot of other formats. It is used within the data model `user/model.rs` and works.
3. __parkinglot__. It is a library that provides implementations of concurrency mechanisms like locks and mutex at low level and in a recursive and fast way. It is used inside `user/model.rs` for the access to the song list.


### Containers

I need to be able to put my application in a container and then be able to establish a proper orchestration of my microservices.

The alternatives were:

1. Docker
2. LXC

Docker (application container) offers a portable development between machines, versioning and reusable libraries, as well as reusability between components, while `LXC` (system container) offers great features in UNIX environments, it does not have facilities such as continuous integration or the orchestration system that offers `Docker`. Therefore we chose `Docker`.

In the `Dockerfile` of the repository [music_store](https://github.com/pepitoenpeligro/music_store) you can see how easy it is to configure a container of our application. It details how to use the `Dockerfile` within the `Readme.md` of that repository.


### Databases

Taking into account that our application is designed to support a large volume of connections, non-relational databases (mysql, maridb, postgresql) are discarded.

Therefore, among the most used nosql alternatives we could value between `MongoDB` and `Redis`.

MongoDB works with JSON documents natively, allows a flexible structure and is easily scalable horizontally.
Redis works with key-value pairs and does not have a query language, but when executed in memory, it allows us to work with very little latency and greater speed.

Therefore, I propose MongoDB as the main database, and Redis as a cache system.

In the repository project, I have not used MongoDB, but I have used it in previous projects such as

1. [PER Test](https://testalv.herokuapp.com)
2. [VCSserver](https://github.com/pepitoenpeligro/VCSserver)

As far as I know its use, integration and deployment.


### Version Control System

The default version control system for `Rust` is `git`. I have used `git` before and also its competitors as `subversion` or `mercurial`. I see more scope for the project as a programmer using git.




## References
* [warp](https://github.com/seanmonstar/warp)
* [serde](https://github.com/serde-rs/serde)
* [parking_lot](https://github.com/Amanieu/parking_lot)