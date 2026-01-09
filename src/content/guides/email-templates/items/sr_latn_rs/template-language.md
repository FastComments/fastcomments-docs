FastComments Email Templates koristi [EJS jezik za šablone](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Primer sintakse je `<%= object.someValue %>` za ispis promenljivih, a uslovne izjave se mogu napisati ovako:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Ispis sirovog HTML-a, na primer za sadržaj komentara, radi se ovako: `<%- comment.commentHTML %>`. Obratite pažnju na `-` umesto `=`.

Pogledajte gore navedeni link za dodatnu dokumentaciju o sintaksi.