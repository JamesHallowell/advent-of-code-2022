import std/algorithm
import std/sequtils
import std/strutils

const example = staticRead("input.txt")

let part1 = example
    .split("\n\n")
    .mapIt(it.splitLines())
    .mapIt(it.filterIt(not it.isEmptyOrWhitespace()))
    .mapIt(it.mapIt(it.parseInt()))
    .mapIt(it.foldl(a + b, 0))
    .max()

echo part1

let part2 = example
    .split("\n\n")
    .mapIt(it.splitLines())
    .mapIt(it.filterIt(not it.isEmptyOrWhitespace()))
    .mapIt(it.mapIt(it.parseInt()))
    .mapIt(it.foldl(a + b, 0))
    .sorted(Descending)[0 .. 2]
    .foldl(a + b, 0)

echo part2