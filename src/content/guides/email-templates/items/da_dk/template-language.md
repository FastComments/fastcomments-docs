---
FastComments Email Templates bruger [EJS skabelonsprog](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Eksempel på syntaks er `<%= object.someValue %>` for at udskrive variabler, og betingede udsagn kan laves således:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Udskrivning af rå HTML, som for kommentarindholdet, gøres således: `<%- comment.commentHTML %>`. Bemærk `-` i stedet for `=`.

Se linket ovenfor for yderligere dokumentation om syntaks.
---