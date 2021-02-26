[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

The comment count displayed at the top of the comment widget can either show all "top-level" comments, meaning those replies that
are replies directly to the page or article itself, or it can be a count of **all** nested comments. By default, it is a count of the former.

We can change this, so that it is a count of **all** nested comments by setting the **countAll** flag to true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

This currently cannot be customized without code changes.
