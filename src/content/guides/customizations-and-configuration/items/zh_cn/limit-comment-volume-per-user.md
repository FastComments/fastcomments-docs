默认情况下，每个用户在同一分钟内最多可以提交 `5 comments`。

这通过用户 ID、匿名用户 ID 和 IP 地址（已哈希）进行跟踪。

这可以在小部件自定义页面上无需代码进行自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='小部件自定义页面上每分钟最大评论数字段，默认设置为 5'; title='限制每位用户的评论量' app-screenshot-end]

注意，如果您使用评论创建 API，可能需要在请求中向我们的后端传递用户的原始 `ip` 地址，以便对每个用户进行速率限制，已应用  
于每个用户，而不是对您的账户全局限制。