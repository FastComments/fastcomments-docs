FastComments-E-Mail-Vorlagen verwenden die [EJS-Template-Sprache](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Die Beispielsyntax ist `<%= object.someValue %>` zum Ausgeben von Variablen, und bedingte Anweisungen können wie folgt geschrieben werden:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Die Ausgabe von rohem HTML, z. B. für den Kommentarinhalt, erfolgt so: `<%- comment.commentHTML %>`. Beachten Sie das `-` anstelle des `=`.

Siehe den obigen Link für weitere Dokumentation zur Syntax.