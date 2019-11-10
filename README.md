## async-reqwest
> Using the new async/.await syntax and comparing the reqwest blocking client to the async one


This is just an exmaple of trying out Rust's new async/.await syntax with [reqwest](https://docs.rs/reqwest/0.10.0-alpha.1/reqwest/).
Timing the blocking version vs the async version while connected to a 4G hotspot:

|Run #|Blocking|Async|
|:-----:|:--------:|:-----:|
|1|0.821|0.364|
|2|1.501|0.344|
|3|0.767|0.326|
|4|1.038|0.293|
|5|1.110|0.451|
---
Note: This should not be taken as an example of best practices with using Futures or the async/.await syntax.
