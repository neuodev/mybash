# MyBash

A very minimalistic programming language built with Rust

<p align="center"> 
    <img src="./mybash
.png" alt="My Bash" title="My Bash">
</p>

## Example

```bash
# mybash ./foo.mb bar baz
# This is a comment
# If statments and exper evaluation
if $1 = "bar"
do echo "I got bar"
else
do echo "I got baz"
endif

# Variables
name: str = "Jone"
age: int = 31
math_expr = 12 / 2 + 1 # 7
echo math_expr
```

# Todo

1. Make program `args` and `env` variables available using the `$` syntax
2. Comments
3. Math expr evaluation
4. Support string concatnation with the echo command
