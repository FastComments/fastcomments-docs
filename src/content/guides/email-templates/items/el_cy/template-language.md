Το FastComments Email Templates χρησιμοποιεί τη [γλώσσα προτύπων EJS](https://github.com/mde/ejs/blob/main/docs/syntax.md).

Παράδειγμα σύνταξης είναι `<%= object.someValue %>` για την εμφάνιση μεταβλητών, και οι δηλώσεις υπό συνθήκη μπορούν να γίνουν ως εξής:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Η εκτύπωση ακατέργαστου HTML, όπως για το περιεχόμενο του σχολίου, γίνεται ως εξής: `<%- comment.commentHTML %>`. Σημειώστε το `-` αντί για το `=`.

Ανατρέξτε στον παραπάνω σύνδεσμο για περισσότερη τεκμηρίωση σχετικά με τη σύνταξη.