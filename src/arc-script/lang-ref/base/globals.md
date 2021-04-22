# Globals

A **global** is an *immutable* variable which can be referenced by-path.

<pre>
<code>Item ::=
  | <'val'> Name <'='> Value <';'>  # Global
  | ...
</code>
</pre>

## Example

The following code initializes two global variables, one by a literal, and another by calling a function.

```text
val pi = 3.14;
val fib10 = fib(10);

fun test() {
    print(pi);
    print(fib10);
}
```
