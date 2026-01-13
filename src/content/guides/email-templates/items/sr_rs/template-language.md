FastComments Email Templates користи [EJS језик шаблона](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Пример синтаксе је `<%= object.someValue %>` за испис променљивих, а условне наредбе могу се писати на следећи начин:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Испис чистог HTML-а, као што је садржај коментара, ради се овако: `<%- comment.commentHTML %>`. Обратите пажњу на `-` уместо `=`.

Погледајте горе наведени линк за додатну документацију о синтакси.

---