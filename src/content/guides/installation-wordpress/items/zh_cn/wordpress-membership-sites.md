FastComments 通过所谓的 SSO（单点登录）与仅限会员的网站协同工作。您的会员登录您的 WordPress 网站后，无需担心在 FastComments 创建账户或使用社交媒体登录即可评论。如果您的会员（包括管理员）已登录您的 WordPress 网站，他们就可以发表评论。您的管理员和版主也可以直接在您的 WordPress 博客文章中审核评论。

<sup>（可选）</sup>请记住，同时将您的管理员添加到 [用户与管理员](https://fastcomments.com/auth/my-account/users)，将版主添加到 [评论版主](https://fastcomments.com/auth/my-account/moderate-comments/moderators)，以改善他们的体验并为版主启用统计跟踪。

可以通过进入插件仪表板并点击"SSO Settings"来启用 SSO。

您首先必须启用您网站的"任何人都可以注册"功能。

每次用户查看评论串时，所有用户信息都会通过 [HMAC](https://en.wikipedia.org/wiki/HMAC) 无缝且安全地传输到 FastComments。

无需运行初始或持续的同步来将您的会员复制到 FastComments 服务器。当他们通过打开博客文章查看评论串时，这会自动完成。

## 名称和头像

当用户查看任何评论串时，插件会自动更新他们在 FastComments 中所有评论上的显示名称和头像。头像通过 Gravatar 或 WordPress 中的任何头像管理插件提供支持，因为插件会调用 `get_avatar_url`。