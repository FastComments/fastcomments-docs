FastComments Email Templates utilizza il [linguaggio di template EJS](https://github.com/mde/ejs/blob/main/docs/syntax.md).

La sintassi di esempio è `<%= object.someValue %>` per stampare variabili, e le istruzioni condizionali possono essere fatte così:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

La stampa di HTML non elaborato, come per il contenuto del commento, si fa così: `<%- comment.commentHTML %>`. Nota il `-` invece di `=`.

Consulta il link sopra per ulteriore documentazione sulla sintassi.