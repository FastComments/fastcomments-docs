FastComments Email Templates uses the [EJS template language](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Example syntax is `<%= object.someValue %>` to print variables, and conditional statements can be done as so:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Printing raw HTML, like for the comment content, is done like so: `<%- comment.commentHTML %>`. Note the `-` instead of the `=`.

Refer to the above link for further documentation on syntax.