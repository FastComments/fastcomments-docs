FastComments Email predlošci koriste [EJS jezik predložaka](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Primjer sintakse je `<%= object.someValue %>` za ispis varijabli, a uvjetne izjave mogu se napraviti ovako:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Ispisivanje sirovog HTML-a, kao za sadržaj komentara, radi se ovako: `<%- comment.commentHTML %>`. Primijetite `-` umjesto `=`.

Pogledajte gornju poveznicu za daljnju dokumentaciju o sintaksi.