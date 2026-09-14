---
一旦关闭 `demo` 租户，小部件可能会因授权错误而拒绝加载。这是因为 FastComments 不知道它应该允许在该域上使用您的账户。

[前往此处将您的站点添加到您的账户。](https://fastcomments.com/auth/my-account/configure-domains)

Val Town 在这里值得再次关注，因为一个 val 可以通过多个主机名访问：

- 每个 HTTP val 都有一个较长的默认端点，<org>--<id>.web.val.run。
- 声明自定义子域名会添加 <name>.val.run。
- [自定义域名](https://docs.val.town/vals/http/custom-domains/) 会再添加一个。
- 分支会拥有自己的 URL。

添加您实际提供小部件服务的所有主机名。如果在设置完成后再声明子域名，请也将其添加，否则小部件将在旧 URL 上工作，而在新 URL 上失败。

---