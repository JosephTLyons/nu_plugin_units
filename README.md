# nu_plugin_units

A [Nushell](https://www.nushell.sh) plugin for easily converting between common units.

```shell
〉units -c time -u years -v 1
╭────┬──────────────┬────────────────────────╮
│  # │     unit     │         value          │
├────┼──────────────┼────────────────────────┤
│  0 │ centuries    │                 0.0100 │
│  1 │ days         │               365.0000 │
│  2 │ decades      │                 0.1000 │
│  3 │ hours        │              8760.0000 │
│  4 │ microseconds │    31540000000000.0000 │
│  5 │ milliseconds │       31556952000.0000 │
│  6 │ minutes      │            525600.0000 │
│  7 │ months       │                12.0000 │
│  8 │ nanoseconds  │ 31540000000000000.0000 │
│  9 │ seconds      │          31536000.0000 │
│ 10 │ weeks        │                52.1430 │
│ 11 │ years        │                 1.0000 │
╰────┴──────────────┴────────────────────────╯
```

You may want to consider adjusting the value of `float_precision` in your nushell config file.

```nu
$env.config = {
    float_precision: 4
    ...
}
```
