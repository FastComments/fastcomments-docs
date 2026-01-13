---
FastComments E-posta Şablonları, [EJS şablon dili](https://github.com/mde/ejs/blob/main/docs/syntax.md) kullanır.

Değişkenleri yazdırmak için örnek sözdizimi `<%= object.someValue %>` şeklindedir ve koşullu ifadeler şu şekilde yapılabilir:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

Yorum içeriği gibi ham HTML yazdırmak şu şekilde yapılır: `<%- comment.commentHTML %>`. Dikkat edin, `-` `=` yerine kullanılır.

Sözdizimi ile ilgili daha fazla belge için yukarıdaki bağlantıya bakın.

---