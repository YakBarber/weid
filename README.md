# weid

## What is weid?

`weid` is a modular, configurable tool for sequentially prompting the user to answer questions and perform actions.

Realistically, `weid` can be a lot of things:

- A terminal-based menu frontend
- A tool for reviewing/reacting to something
- A quiz/flashcard interface
- A text-based adventure game engine

## Expectations

`weid` is currently in "early alpha". At the moment, it is mostly a vehicle for the [Pinboard modification example](examples/pbin), as a demonstration of some of `weid`'s goals via its use as a library.

The standalone `weid` application, while functional, is not all that useful yet.

It should be expected that things will break, not work, and change as `weid` matures.

## Installation

You can build `weid` by cloning this repository and using the standard Rust tooling: 

    git clone https://github.com/yakbarber/weid
    cd weid
    cargo build
    cargo install

The unit tests can be run via `cargo test`.

## Usage

### As a standalone application

`weid` can be called from the command line with the following options:

- `-q TEXT`, `--query=TEXT`: Define a new query (a question you want `weid` to ask you). The text supplied to this argument can be in markdown format.
- `-a TEXT`, `--answer=TEXT`: Define a new answer (an option you have when `weid` asks you a question)
- `-o TEXT`, `--outcome=TEXT`: Define a new outcome (something that happens when you pick a given answer.

These arguments are *position sensitive*. Examples are helpful.

This will create a single query with two possible answers:

    weid -q "How are you feeling?" -a "good" -a "bad" 

The following will create two queries, the first with three possible answers, and the second with only one. The `-a "spicy!"` argument occurs before any queries are defined, so it is a general answer that is available to all queries. The other two answers only apply to the query that preceded them.

    weid -a "spicy!" -q "How are you feeling?" -a "good" -a "bad" -q "What rhymes with klicy?"

The result will be something like:

    How are you feeling?
    [1] good
    [2] bad
    [3] spicy!

    ...

    What rhymes with klicy?
    [1] spicy!

Similarly, outcomes associate with the answer that immediately precedes them in the argument order. Outcomes cannot be set without an associated answer. Observe:

    weid -q "Do you want to run `ls`?" -a "yes" -o "ls" -a "no"

If you answer "yes" to the resulting prompt, then `weid` will run the `ls` command and dump the result to stdout. 

### As a Library

Until better docs are made, the best reference for using `weid` as a library besides the source itself is the [Pinboard example](examples/pbin). This demonstrates more effective usage of the internal mechanisms to define queries programmatically. It also utilizes markdown to format the queries.

## Design

`weid`'s architecture is built from five main objects:

- `struct Query` - Represents the question being asked. Stores `Answer`s.
- `struct Answer` - Represents the answer that you choose. Stores `Outcome`s.
- `enum Outcome` - Represents something that happens when an answer is chosen. At the moment, this is limited to a string representing a shell command. The library version of `weid` also includes the ability to run arbitrary closures and will eventually be able to modify the `QueryList`, described below, which theoretically will allow the user to modify which `Query` is asked next, among other niftyness.
- `struct QueryList` - Stores the `Query`s (and the `Answer`s/`Outcome`s they contain) for the current session. It maintains awareness of which `Query` is which.
- `struct Querier` - The session state. Perhaps I should have called it `Session`. This manages the `QueryList` and remembers which `Query`s have been asked already.

All of this is subject to change, but hopefully the intent and direction of this project is somewhat elucidated.

## THE FUTURE

Planned additions to `weid` mostly focus on improving interaction and enabling external scripting. This may include the ability to interact with `weid` via a FIFO or secondary stdin stream, in addition to the CLI arguments. This will necessarily include a better command definition/description format.

More examples, documentation, and help text are also necessary.

## Name

As I'm sure you have already guessed, `weid`'s name is derived from the reconstructed Proto-Indo-European root _\*weid-_, which means "to see" or "to know".

_\*weid-_ is an ancestor to such relevant words as _advise_, _review_, _wisdom_, _interview_, _guide_, _idea_, and _history_, and such irrelevant words as _penguin_, _prudent_, _idol_, _envy_, and _twit_.

I pronounce _\*weid-_ as _wi&#720;d_, like "weed."

