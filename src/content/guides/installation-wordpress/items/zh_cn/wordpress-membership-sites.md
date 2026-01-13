FastComments 通过使用被称为 SSO（单点登录）的机制与仅限会员的网站配合使用。您的成员在您的 WordPress 站点登录，但
不必为了发表评论而担心在 FastComments 创建帐号或使用社交媒体登录。如果您的成员（包括管理员）已登录到
您的 WordPress 站点，他们就能发表评论。您的管理员和版主也可以直接在您的 WordPress 博客文章中进行评论管理。

<sup>(可选)</sup> 请记得还要将您的管理员添加到 [User & Administrators](https://fastcomments.com/auth/my-account/users) 和版主添加到 [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
以改善他们的体验并为版主启用统计跟踪。

SSO can be enabled by going to the plugin dashboard and clicking "SSO Settings".

You will first have to enable the "Anyone can Register" feature of your site.

All user information is seamlessly and securely transferred to FastComments each time a user views a comment thread via [HMAC](https://en.wikipedia.org/wiki/HMAC).

There is no initial or continuous sync that needs to be run to copy your members over to FastComments servers. This is automatically done when they view comment threads by opening a blog post.

## 名称和头像

The plugin will automatically update the user's display name and avatar on all their comments within FastComments when they view
any comment thread. Avatars are supported via Gravatar or any avatar management plugin within WordPress as the plugin will call `get_avatar_url`.