# nu_plugin_units

A [Nushell](https://www.nushell.sh) plugin for easily converting between common units.

```shell
〉units -c time -u minutes -v 3
╭────┬──────────────┬───────────────────╮
│  # │     unit     │       value       │
├────┼──────────────┼───────────────────┤
│  0 │ centuries    │            0.0000 │
│  1 │ days         │            0.0021 │
│  2 │ decades      │            0.0000 │
│  3 │ hours        │            0.0500 │
│  4 │ microseconds │    180000000.0000 │
│  5 │ milliseconds │       180000.0000 │
│  6 │ minutes      │            3.0000 │
│  7 │ months       │            0.0001 │
│  8 │ nanoseconds  │ 180000000000.0000 │
│  9 │ seconds      │          180.0000 │
│ 10 │ weeks        │            0.0003 │
│ 11 │ years        │            0.0000 │
╰────┴──────────────┴───────────────────╯
```

You may want to consider adjusting the value of `float_precision` in your nushell config file.

```nu
$env.config = {
    float_precision: 4
    ...
}
```
