FastComments Email Templates usa el [lenguaje de plantillas EJS](https://ejs.co/#docs).

La sintaxis de ejemplo es `<%= object.someValue %>` para imprimir variables, y las sentencias condicionales pueden hacerse así:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

La impresión de HTML sin escapar, como para el contenido del comentario, se hace así: `<%- comment.commentHTML %>`. Nótese el `-` en lugar de `=`.

Consulte el enlace anterior para más documentación sobre la sintaxis.