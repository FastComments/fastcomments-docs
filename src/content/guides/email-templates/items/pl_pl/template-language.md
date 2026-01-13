FastComments Email Templates używa [języka szablonów EJS](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Przykładowa składnia to `<%= object.someValue %>` do wypisywania zmiennych, a instrukcje warunkowe można zapisać w ten sposób:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Aby wypisać surowy HTML, na przykład zawartość komentarza, użyj: `<%- comment.commentHTML %>`. Zwróć uwagę na `-` zamiast `=`.

Szczegółową dokumentację składni znajdziesz w powyższym linku.

---