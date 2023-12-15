FastComments uses a combination of Markdown and a simple subset of HTML to allow writing
comments.

A nice resource for [Markdown syntax is here](https://www.markdownguide.org/cheat-sheet/).

For most common formatting, you can use the formatting toolbar to do things like bold or italicize text.

Some syntax examples are:

- Bold:
  - `**bold**`
  - `<b>bold text</b>`
- Italics:
  - `*italics*`
  - `<i>italic text</i>`
- Strikethrough:
  - `~~strikethrough~~`
  - `<strike>strikethrough</strike>`
- Quotes:
  - `> quote`

Note that Markdown headings are not supported.

Links can be written as their raw URL, without HTML or other syntax, and by default the link
will be turned into a clickable link with `target="nofollow noopener"` to discourage spammers. Some sites
may choose to disable automatic link creation.

Ordered lists can be written like:

```
1. One Item.
2. Second Item.
3. Third Item.
```

The same goes for bulleted lists:

```
- Some point.
- Some other point.
```

For programming-oriented communities, code can be shared by pasting it in the comment area, and the language will
be auto-detected and formatted. Code can be added in `<code></code>` tags or Markdown code fences with backticks.
