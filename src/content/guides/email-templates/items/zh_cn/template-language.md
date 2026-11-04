FastComments 邮件模板使用 [EJS 模板语言](https://ejs.co/#docs)。

语法示例：`<%= object.someValue %>` 用于输出变量，条件语句可以这样写：

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

输出原始 HTML（例如评论内容）的方式如下：`<%- comment.commentHTML %>`。注意使用的是 `-` 而不是 `=`。

有关语法的更多文档，请参考上面的链接。