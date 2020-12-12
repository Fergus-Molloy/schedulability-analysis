# Scheduling

This is a command line utility to do some of the more tedious calculations needed for analysing real time systems

## Usage

To use this you *must* supply a csv file containing the tasks you want analysed.
They should be in the same form as the example in the tasks file.
If deadlines are implicit you need not put data there but the column must be present.

The flags on this are a bit weird. The `-u` option or `--utilisation` **must** go at the end since it has an optional parameter.
This means it's a good idea to do `cr -- <INPUT> [FLAGS][PARAMETER]`.
To see all options simply run `cr -- -h`.

## Features

Currently the features implemented are:

- Response time analysis (not including jitter or any more advanced stuff)
- L&L utilisation test (including being able to give the number of families)
