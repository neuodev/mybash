# MyBash

A very minimalistic programming language built with Rust

<p align="center"> 
    <img src="./mybash.png" alt="My Bash" title="My Bash">
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
is_awesome: bool = true
math_expr = 12 / 2 + 1 # 7
echo math_expr
```

## Examples

<details>
<summary>Variable declaration with basic if statment</summary>

```bash
# mybash script.mb

age: int = 30
echo age
if age > 40
do echo "I am old"
else
do echo "I am still young"
endif
```

#### Output

```bash
30
I am still young
```

</details>

<details>
<summary>Access positional arguments and echo it to the stdout</summary>

```bash
# mybash script_5.mb me
echo $1

if $1 == 'me'
do echo "Hello, Ahmed"
else
do echo "Hi, :) ${1} ✋"
endif
```

#### Output

```bash
Ahmed
Hi, :) Ahmed ✋
```

</details>
<details>
<summary>Access evn variables (with comments)</summary>

```bash
# Echo env variables to the stdout
# example: mybash ./sript_6.mb foo bar baz
echo $PATH
echo $PWD # Current working directory

echo $0  # Should display the script name  (script_6.mb)
echo $1  # Should display the first arg (foo)
echo $2  # Should display the second arg (bar)

```

</details>

<details>
<summary>Evaluate math expressions and echo it to the stdout</summary>

```bash
res: int = (12 + 12) / 4
echo "(12 + 12) / 4 ⏬"
echo res
```

#### Output

```bash
(12 + 12) / 4 ⏬
6
```

</details>

<details>
<summary>Variable expansion with echo statments</summary>

```bash
name: str = "Jone"

echo "Hello, $name 🙌"

echo "PATH = ${PATH}"
echo "HOME = $HOME"
echo "PWD = $PWD"
echo "HOSTNAME = $HOSTNAME"
echo "HOSTTYPE = $HOSTTYPE"
```

</details>

<details>
<summary>Variable concatenation</summary>

```bash
f_name: str = "Jone"
l_name: str = "doe"

full_name: str = "${f_name} ${l_name}"

echo "My full name is $full_name"
```

#### Output

```bash
My full name is Jone doe
```

</details>

<details>
<summary>Read stdin</summary>

```bash
name: string = input("What is your name? ")
age: string = input("What is your age? ")
addr: string = input("What is your address? ")

echo "My name is $name"
echo "My age is $age"
echo "I live in $addr"
```

</details>
