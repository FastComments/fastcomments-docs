[related-parameter-start name = 'startingPage'; type = 'string'; related-parameter-end]

When fetching and rendering comments, the comment widget needs to know which page to start on. By default, it starts with
the first page, only rendering that page.

If desired, the exact page to be rendered can be passed to the comment widget as the setting *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Note that pages numbers start from zero, so the above example renders the second page.
