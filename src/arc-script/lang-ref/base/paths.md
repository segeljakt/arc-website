# Paths

A **path** is a list of names which point to an **item** in the module hierarchy.

<pre>
<code>Path ::= <'::'>? (Name <'::'>)* Name
</code>
</pre>

Paths can be both *relative* and *absolute*. Absolute paths are absolute with respect to the root of the module hierarchy. Relative paths are relative with respect to the current namespace.

```text
mod1::mod2::MyItem   # Relative path
::mod1::mod2::MyItem # Absolute path
```
