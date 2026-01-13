---
FastComments E-mailsjablonen gebruiken de [EJS-sjabloontaal](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Voorbeeldsyntaxis is `<%= object.someValue %>` om variabelen weer te geven, en voorwaardelijke instructies kunnen als volgt worden geschreven:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Ruwe HTML weergeven, bijvoorbeeld voor de inhoud van een opmerking, gebeurt als volgt: `<%- comment.commentHTML %>`. Let op het `-` in plaats van `=`.

Raadpleeg de bovenstaande link voor meer documentatie over de syntaxis.

---