Hugo'nun yerleşik Disqus desteğinin yaptığı gibi her gönderiye yorum eklemek için, temanızın single şablonuna (`layouts/_default/single.html`) bir satır ekleyin:

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Partial yalnızca bir `tenantId` yapılandırıldığında render edilir. Bireysel bir sayfada yorumları front matter ile devre dışı bırakın:

```toml
+++
title = "A page with no comments"
comments = false
+++
```