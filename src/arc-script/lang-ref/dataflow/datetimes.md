# Datetimes

A **datetime** (or **time** for short) is a *moment in time*.

<pre>
<code>Literal ::=
  | [0-9]+<'-'>[0-9]+<'-'>[0-9]+                                         # Date
  | [0-9]+<'-'>[0-9]+<'-'>[0-9]+<'T'>[0-9]+<':'>[0-9]+<':'>[0-9]+                    # Date + Time
  | [0-9]+<'-'>[0-9]+<'-'>[0-9]+<'T'>[0-9]+<':'>[0-9]+<':'>[0-9]+(<'+'>|<'-'>)[0-9]+<':'>[0-9]+  # Date + Time + Zone
  | ..

Scalar ::=
  | <'time'>
  | ..
</code>
</pre>

## Examples

Some examples:

```text
2020-04-18
2020-04-18T18:03:01
2020-04-18T18:03:01+02
```
