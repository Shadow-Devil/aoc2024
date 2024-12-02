%dw 2.0
output application/json
import * from dw::core::Strings

var parsed = lines(payload) reduce ((item, acc = {l: [], r: []}) -> do {
    var left = (item substringBefore " ") as Number
    var right = (item substringAfterLast " ") as Number
    ---
    {l: acc.l + left, r: acc.r + right}
})

fun part1() = parsed
    then zip($.l orderBy $, $.r orderBy $) map abs($[0] - $[1])
    then sum($)

fun part2() = do {
    var l = parsed.l groupBy $ pluck [$$, sizeOf($)]
    var r = parsed.r groupBy $ mapObject {"$($$)": sizeOf($)}
    ---
    l map ($[0] as Number * $[1] * r[$[0]] default 0)
    then sum($)
}

---
{
    part1: part1(),
    part2: part2()
}