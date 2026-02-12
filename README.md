# roll

Simple CLI dice roller.

## usage

    roll [DICE]...

Supports standard notation (`NdS`) and advantage/disadvantage (`a`/`d`).

```bash
# normal roll
roll 1d20

# multiple dice
roll 4d6 1d8

# advantage (roll 2 keep highest)
roll 1d20a

# disadvantage (keep lowest)
roll 1d20d

# modifiers (add/subtract from result)
roll 1d20+5
roll 1d8-2
```

Output looks like this:

```text
+-------+--------+
| Die   | Roll   |
+=======+========+
| d20a  | 18 (4) |
| d6    | 5      |
+-------+--------+
| Total | 23     |
+-------+--------+
```

## install

```bash
cargo install alecghost-roll
```

or with this repo checked out locally

```bash
cargo install --path .
```
