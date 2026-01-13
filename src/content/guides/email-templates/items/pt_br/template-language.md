FastComments Email Templates usa a [linguagem de template EJS](https://github.com/mde/ejs/blob/main/docs/syntax.md).

A sintaxe de exemplo é `<%= object.someValue %>` para imprimir variáveis, e declarações condicionais podem ser feitas assim:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Renderizar HTML bruto, como para o conteúdo do comentário, é feito assim: `<%- comment.commentHTML %>`. Observe o `-` em vez de `=`.

Consulte o link acima para mais documentação sobre a sintaxe.