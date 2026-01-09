---
FastComments Email Templates は [EJS テンプレート言語](https://github.com/mde/ejs/blob/main/docs/syntax.md) を使用します。

変数を出力する例は `<%= object.someValue %>` で、条件文は次のように記述できます:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

コメント内容のような生のHTMLを出力するには `<%- comment.commentHTML %>` のようにします。 `=` の代わりに `-` を使う点に注意してください。

構文の詳細は上記のリンクを参照してください。

---