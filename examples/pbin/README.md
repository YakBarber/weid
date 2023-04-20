# Pinboard editing with weid

## What is this

Pinboard (https://pinboard.in), if you aren't familiar, is a link aggregation website.

This example demonstrates a minimum viable application using `weid` as a library to step through your saved pinboard links and provide you with an opportunity to edit them, in a menu-driven style.

The options presented for each link are:

- edit description
- edit the title
- add/remove tags
- mark/unmark the link as "to read"


## Usage

Using this example requires you have a Pinboard account.

First, you will need to obtain your Pinboard API token, which you can find in the Pinboard settings under "Password" ([here](https://pinboard.in/settings/password)). It looks like this: `username:0123456789ABCDEFFFFF`.

Once you have it, store it in an env variable named `PINBOARD_API_TOKEN`. You can then run this example with the usual `cargo run --example=pbin`.

## Caveats

While this *works on my machine*(TM), there are likely bugs and edge cases that have not been addressed. I make no guarantee of this code's fitness for any purpose. I'm also not affiliated with Pinboard or its creator; I am merely a user of the site.
