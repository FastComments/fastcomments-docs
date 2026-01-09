---
默认情况下，每位用户在同一分钟内最多可以提交 `5 comments`。

这是通过 user id、anon user id 和 ip address（已哈希）进行跟踪。

可以在小部件自定义页面上无需编写代码进行自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

请注意，如果你正在使用 comment creation API，可能需要在请求中将用户的原始 `ip` 地址传递给我们的后端，以便速率限制被
按用户而不是对你的帐户全局应用。
---