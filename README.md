# Planner [![Status][status-img]][status-url]

Planner is a tool for constructing floorplans.

## Usage

```
$ planner --help
Usage: planner [options]

Options:
    --database <path>        SQLite3 database (required).
    --table <name>           Table containing area estimates (required).
    --cores <number>         Number of cores (required).
    --output (3d-ice|svg)    Output format [default: 3d-ice].

    --help                   Display this message.
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[status-img]: https://travis-ci.org/learning-on-chip/planner.svg?branch=master
[status-url]: https://travis-ci.org/learning-on-chip/planner
