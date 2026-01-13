Шаблоны писем FastComments используют [язык шаблонов EJS](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Пример синтаксиса: `<%= object.someValue %>` для вывода переменных, а условные операторы можно записать так:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Вывод «сырого» HTML, например содержимого комментария, выполняется так: `<%- comment.commentHTML %>`. Обратите внимание на `-` вместо `=`.

См. приведённую выше ссылку для получения дополнительной документации по синтаксису.