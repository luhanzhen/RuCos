
# <font color = "#FF8000">Ru</font>st <font color = "#FF0000">Co</font>nstraint <font color = "#FF0000">S</font>olver(<font color = "#FF8000">Ru</font><font color = "#FF0000">Cos</font>)


# <font color = "#FF8000">Ru</font><font color = "#FF0000">Cos</font> is an extremely high-performance parallel constraint solver implemented by Rust.

# <font color = "#FF8000">Ru</font><font color = "#FF0000">Cos</font>  is implemented by safe, elegant and lower-level code with zero-cost abstraction. 

# <font color = "#FF8000">Ru</font><font color = "#FF0000">Cos</font> is a total Test-Driven developed solver.

# I have implemented an XCSP3 parser whose repo is [xcsp3-rust](https://github.com/luhanzhen/xcsp3-rust).

# I will implement the main part of the solver and some typical types of constraints first.

# The process of implementing the solver could take years, but I will keep updating....


# Usage

## define the problem
### you can define the problem with the following code:
```rust
fn main(){
    let  problem = Problem::new();
}
```
### or the following code:
```rust
fn main(){
    let problem = Default::default();
}
```
### or the following code:
```rust
fn main(){
    let problem = problem!();
}
```

## define the variables
```rust
fn main(){
    let problem = problem!();
    problem += var!("var1"; 1=> 10);
    problem += var!("var2";1=> 10);
    problem += var!("var3";1=> 10);
    problem += all_different!(
                problem["var1"].clone(),
                problem["var2"].clone(),
                problem["var3"].clone());
}
```
### you can define the variables with following code:
```rust
fn main() {
    let mut problem = problem!();
    let v1 = var!(&mut problem; "v1"; domain![7, 43, 22, 33, 2234]);
    let v2 = var!(&mut problem; "v2"; domain![7, 43, 22, 33, 2234, 43]);
    let v3 = var!(&mut problem; "v3"; 7, 43, 22, 33, 2234, 43);
    let v4 = var!(&mut problem; "v4"; 1=>100);
    let v4 = var!(&mut problem; "v5"; domain![1=>1000]);
    
    problem += var!("v4"; 7=> 43);
    problem += var!("v5";7, 54, 65, 43);
    problem += var!("v6"; 7=> 43);
    
    problem += bool!("v5");
    problem += bool!("v6");
    problem += bool!("vbool_1");
    problem += bool!("vbool_2");
    problem += bool!();
    
    // you can get the variables by following code:
    println!("{}",problem["v1"]);
    println!("{}",problem["v4"]);
    println!("{}",problem["v5"]);
    println!("{}",problem[0]);
}
```
## define the constraints
### you can define the constraint with following code:
```rust
fn main() {
    let mut problem = problem!();
    for i in 0..n {
        problem += var!(&format!("row_{}"; &i), 0=>(n as i32));
    }
    
    problem += all_different!(
                &problem[format!("row_{}", i).as_str()],
                &problem[i],
            );
}

```

## the n-queen problem can be modelled by following code:
```rust
fn n_queens(n: usize) -> Problem {
    let mut problem = problem!();
    for i in 0..n {
        problem += var!(&format!("row_{}"; &i), 0=>(n as i32));
    }

    for i in 0..n {
        for j in i + 1..n {
            problem += all_different!(
                &problem[format!("row_{}", i).as_str()],
                &problem[j]
            );
        }
    }
    problem
}
```