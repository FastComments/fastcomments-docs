---
Les modèles d'e-mails FastComments utilisent le [EJS template language](https://github.com/mde/ejs/blob/main/docs/syntax.md).

La syntaxe d'exemple est `<%= object.someValue %>` pour afficher des variables, et les instructions conditionnelles peuvent être écrites ainsi :

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Pour afficher du HTML brut, comme le contenu d'un commentaire, faites : `<%- comment.commentHTML %>`. Remarquez le `-` au lieu du `=`.

Consultez le lien ci‑dessus pour plus de documentation sur la syntaxe.

---