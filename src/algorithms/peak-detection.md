# [Peak Detection](https://dl.acm.org/doi/pdf/10.1145/3428251)

> Now, let us consider an algorithm for detect peaks in a stream of numerical values (suppose they are of type `V`). The algorithm searches for the first value that exceeds the threshold `THRESH`. Then, it search for the maximum over the next `#PEAK_CNT` elements, which is considered a peak. After that, the algorithm silences detection for `#SILENCE_CNT` elements to avoid a duplicate detection. This process is repeated indefinitely in order to detect all peaks.

## Implementation (Arc-Script)

```
type V = i32;
type N = u32;

fun query(stream: ~V, PEAK_CNT: N, THRESH: V, SILENCE_CNT: N): ~V {
    iterate(stream,
        seq(Search(fun(v): v > THRESH),
            Take(PEAK_CNT)
                |> Reduce(fun(x, y): if x > y { x } else { y })
                |> Ignore(SILENCE_CNT)
        )
    )
}
```

## Implementation ([StreamQL](https://dl.acm.org/doi/pdf/10.1145/3428251))

```
Q<V,V> start = search(v -> v > THRESH);
Q<V,V> take = take(PEAK_CNT);
Q<V,V> max = reduce((x, y) -> (y > x) ? y : x);
Q<V,V> find1 = pipeline(seq(start, take), max);
Q<V,V> silence = ignore(SILENCE_CNT);
Q<V,V> query = iterate(seq(find1, silence));
```
