---
תבניות הדוא"ל של FastComments משתמשות ב[שפת התבניות EJS](https://github.com/mde/ejs/blob/main/docs/syntax.md).

דוגמת התחביר היא `<%= object.someValue %>` להדפסת משתנים, ומשפטי תנאי יכולים להיעשות כך:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

הדפסת HTML גולמי, כגון עבור תוכן התגובה, נעשית כך: `<%- comment.commentHTML %>`. שים לב ל-`-` במקום `=`.

עיין בקישור שלמעלה לתיעוד נוסף על התחביר.

---