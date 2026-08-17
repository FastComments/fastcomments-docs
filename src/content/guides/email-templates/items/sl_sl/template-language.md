FastComments Email Templates uporablja [jezik predlog EJS](https://ejs.co/#docs).

Primer sintakse je `<%= object.someValue %>` za izpis spremenljivk, pogojne izjave pa lahko naredite tako:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Izpis surovega HTML, na primer za vsebino komentarja, se naredi takole: `<%- comment.commentHTML %>`. Upoštevajte `-` namesto `=`.

Za nadaljnjo dokumentacijo o sintaksi glejte zgornjo povezavo.