
# <font color = "#FF8000">Ru</font>st <font color = "#FF0000">Co</font>nstraint <font color = "#FF0000">S</font>olver(<font color = "#FF8000">Ru</font><font color = "#FF0000">Cos</font>)


# <font color = "#FF8000">Ru</font><font color = "#FF0000">Cos</font> is an extremely high-performance parallel constraint solver implemented by Rust.

# <font color = "#FF8000">Ru</font><font color = "#FF0000">Cos</font>  is implemented by safe, elegant and lower-level code with zero-cost abstraction. 

# <font color = "#FF8000">Ru</font><font color = "#FF0000">Cos</font> is a total Test-Driven developed solver.

# I have implemented an XCSP3 parser whose repo is [xcsp3-rust](https://github.com/luhanzhen/xcsp3-rust).

# I will implement the main part of the solver and some typical types of constraints first.

# The process of implementing the solver could take years, but I will keep updating....


## the n-queen problem can be modelled by following code:
```rust
fn n_queens(n: usize) -> Problem {
    let mut problem = problem!();
    let mut vars = vec![];
    for i in 0..n {
        let var = var!(&mut problem,&format!("row_{}", &i), 0=>(n as i32));
        vars.push(var);
    }
    for i in 0..n {
        for j in i + 1..n {
            problem += all_different!(vars[i].clone(), vars[j].clone());
        }
    }
    problem
}

```