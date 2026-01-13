Disqus 与 FastComments Secure SSO 之间最大的区别是 Disqus 使用 SHA1 进行加密，而我们使用 SHA256。

这意味着从 Disqus 迁移很容易 - 将所使用的哈希算法从 SHA1 改为 SHA256 并更新传递给 UI 的属性名称。