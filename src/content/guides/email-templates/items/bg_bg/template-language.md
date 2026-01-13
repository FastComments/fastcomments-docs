---
FastComments Email Templates използва [EJS език за шаблони](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Примерен синтаксис е `<%= object.someValue %>` за извеждане на променливи, а условни изрази могат да се направят по следния начин:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Извеждането на суров HTML, например съдържанието на коментара, се прави така: `<%- comment.commentHTML %>`. Обърнете внимание на `-` вместо `=`.

Вижте горния линк за допълнителна документация относно синтаксиса.

---