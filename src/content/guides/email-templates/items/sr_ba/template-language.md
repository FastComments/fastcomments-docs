---
FastComments Email Templates користи [EJS језик шаблона](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Пример синтаксе је `<%= object.someValue %>` за испис променљивих, а условне изјаве могу се написати на следећи начин:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Испис необрађеног HTML-а, као за садржај коментара, ради се овако: `<%- comment.commentHTML %>`. Обратите пажњу на `-` уместо `=`.

Погледајте горе наведену везу за даљу документацију о синтакси.

---