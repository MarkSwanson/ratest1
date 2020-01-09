When editing this file for a minute I get an error message - something about being disconnected to the ra server.

## Timing - Startup
$ nvim src/lib.rs
RA runs for about 103 seconds.

## Timing - gd (Goto Definition) std::fs::File - line 12
1st try: 70 seconds

2nd try: 10 seconds.

3rd try: 10 seconds.

4th try: instant.

## Timing - gd (Goto Definition) std::backtrace::Backtrace - line 34

1st try: instant.

## Timing - gd (Goto Definition) io::BufWriter - line 12

1st try: instant.

## Timing - gd (Goto Definition) String - line 107

1st try: 10 seconds

2nd try: 3 seconds

3rd try: 12 seconds

4th try: 12 seconds


## 'gd' Questions

It looks like rust-analyzer mostly parses enough rust code to satisfy the current query, and not the whole code base.
It's strange that gd 'String' always takes 12 seconds.

I'd really prefer it if rust-analyzer parsed all 141 packages on startup. Rationale:
1. I can do useful things while waiting for rust-analyzer to parse things when I start vim.
2. I don't care how long it takes - I can leave my vim window open forever via mosh.
3. I don't care how much memory it takes.


## Timing - Code Completion when creating a String in testT() : let s = String::\<C-x C-o\>

Each time code completion takes 18 seconds.






