---
FastComments 電子郵件範本使用 [EJS 範本語言](https://github.com/mde/ejs/blob/main/docs/syntax.md)。

範例語法為 `<%= object.someValue %>` 用於輸出變數，條件敘述可以如下撰寫：

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

輸出原始 HTML（例如評論內容）可使用 `<%- comment.commentHTML %>`。注意這裡使用 `-` 而非 `=`。

有關語法的更多文件，請參考上方連結。

---