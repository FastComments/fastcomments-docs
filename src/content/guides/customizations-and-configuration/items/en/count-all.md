[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

The comment count displayed at the top of the comment widget can either show all "top-level" comments, meaning those replies that
are replies directly to the page or article itself, or it can be a count of **all** nested comments.

By default, this is `true` - it is a count of the latter - all comments. In older versions of the comment widget the
default value is `false`.

We can change the behavior, so that it is a count of **all** nested comments by setting the **countAll** flag to true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

If we wanted the count to reflect only the top level comments, we set the flag to false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

This currently cannot be customized without code changes.
