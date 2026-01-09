FastComments Email Templates користи [EJS језик шаблона](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Пример синтаксе је `<%= object.someValue %>` за испис променљивих, а условне наредбе се могу написати овако:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

За исписивање непрерађеног HTML-а, као што је садржај коментара, користи се: `<%- comment.commentHTML %>`. Обратите пажњу на `-` уместо `=`.

Погледајте горњу везу за додатну документацију о синтакси.