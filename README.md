# CAGrepeat-analyzer
- plot CAG reapeats human genome.
- implemented a tokenizer approach with async to enable faster searches as compared to regex based.
- profiling all CAG repeat across human genome or any other genome at async speed.

```
____      _       ____   ____                                  _
/ ___|    / \     / ___| |  _ \    ___   _ __     ___    __ _  | |_
| |       / _ \   | |  _  | |_) |  / _ \ | '_ \   / _ \  / _` | | __|
| |___   / ___ \  | |_| | |  _ <  |  __/ | |_) | |  __/ | (_| | | |_
\____| /_/   \_\  \____| |_| \_\  \___| | .__/   \___|  \__,_|  \__|
                                        |_|

CAG repeat pattern.
     ************************************************
    Gaurav Sablok, IBCH, PAN, Poznan, Poland,
    https://portal.ichb.pl/laboratory-of-genomics/.
    Email: gsablok@ibch.poznan.pl
    ************************************************

Usage: CAGrepeat-analyzer <COMMAND>

Commands:
cag-repeat       CAG repeat pattern
help             Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version
```

```
./target/debug/CAGrepeat-analyzer cag-repeat ./test-files/test.fa
ENST00000832824.1	3
```

Gaurav Sablok \
Instytut Chemii Bioorganicznej \
Polskiej Akademii Nauk \
ul. Noskowskiego 12/14 | 61-704, Poznań \
Poland
