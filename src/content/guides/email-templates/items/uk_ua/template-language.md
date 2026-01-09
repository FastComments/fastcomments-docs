---
Шаблони електронної пошти FastComments використовують [мову шаблонів EJS](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Приклад синтаксису: `<%= object.someValue %>` для виведення змінних, а умовні вирази можна записати так:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Виведення сирого HTML, наприклад вмісту коментаря, робиться так: `<%- comment.commentHTML %>`. Зверніть увагу на `-` замість `=`.

Детальніше про синтаксис див. у наведеному вище посиланні.

---