# DFA Playground

A small series of projects for experimenting with Deterministic Finite Automata (DFAs) and related scanner/tokenization problems.

## Projects

### dfa-number-scanner (Language: C)

Table-driven DFA that reads from `stdin` and prints recognized integer and float tokens.

Compile with
```bash
gcc number_parser.c -o dfa-number-scanner.out
```

Planned additions:
- comment finder (`//` and `/* */`)
- identifiers/keywords/operators
- string literals + escapes
