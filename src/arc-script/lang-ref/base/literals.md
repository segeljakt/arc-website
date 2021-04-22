# Literals

A **literal** is a value represented as-is in the source code.

<pre>
<code>Literal ::=
  | <'true'> | <'false'>        # Literal boolean
  | <'unit'>                # Literal unit
  | <'\'><' . '>\<''>               # Literal character
  | <'"[^\"]"'>             # Literal string
  | [1-9][0-9]*         # Literal integer
  | [1-9][0-9]*.[0-9]*  # Literal floating point
  | ..
</code>
</pre>
