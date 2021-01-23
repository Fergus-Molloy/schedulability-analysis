# Scheduling

This is a command line utility to do some of the more tedious calculations needed for analysing real time systems

## CSV File

The `tasks_example` file provides a template to use for supplying task sets. The columns *must* remain in that order.
However, the deadline and priority column are optional. If no deadline is given it is assumed that the task set has implicit deadlines (D=T).
If no priority is given then deadline monotonic priority ordering is used (for implicit tasks this is the same as rate monotonic priority ordering).
Priorities must be integers from 1 upwards where 1 is the lowest priority. If a priority of 0 is provided to *any* task then the program will perform
deadline monotonic priority ordering to assign priorities (note omitting all priorities will also do this).


## Usage

To use this you *must* supply a CSV file (see CSV File section)containing the tasks you want analysed.
If deadlines are implicit you need not put data there but the column must be present.

The flags on this are a bit weird. The `-u` option or `--utilisation` **must** go at the end since it has an optional parameter.
This means it's a good idea to do `cr -- <INPUT> [FLAGS][OPTIONS]`.
To see all options simply run `cr -- -h`.

To see how the program has interpreted your input you can pass `-d` or `--debug`

## Example

```bash
$ cr -- tasks_example -u
Utilisation test fails: 0.862 >= 0.757

$ cr -- tasks_example -u=1
Utilisation test passes: 0.862 <= 1.000
```

## Features

Currently the features implemented are:

- Response time analysis (not including jitter or any more advanced stuff)
- L&L utilisation test (including being able to give the number of families)
