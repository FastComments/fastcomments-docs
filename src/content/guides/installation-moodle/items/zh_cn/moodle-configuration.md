插件设置页面位于 **站点管理 > 插件 > 本地插件 > FastComments**。 可用选项如下：

#### Tenant ID

您的 FastComments Tenant ID。可在 <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments 仪表板</a> 的账户设置中找到此项。

#### API Secret

您的 API Secret 密钥，用于 Secure SSO 模式。可在 <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">我的账户 > API Secret</a> 找到此项。

#### SSO Mode

选择用户的身份验证方式。有关每个选项的详细信息，请参见 [SSO 模式](#moodle-sso-modes) 部分。

- **Secure** (recommended) - 服务器端 HMAC-SHA256 签名认证
- **Simple** - 客户端用户数据，无签名
- **None** - 匿名评论，不集成 Moodle 登录

#### Page Contexts

控制评论出现的位置：

- **Course pages** - 课程主页面上的评论
- **Module/activity pages** - 单个活动和资源页面上的评论
- **Both** - 在所有页面类型上显示评论

#### Commenting Style

选择评论体验。有关每种模式的截图，请参见 [评论样式](#moodle-commenting-styles)。

- **Comments** - 在页面内容下方的标准嵌套评论组件
- **Collab Chat** - 基于内联文本选择的讨论，带有在场指示器
- **Both** - 同时启用评论和 Collab Chat

#### CDN URL

FastComments 的 CDN URL。默认为 `https://cdn.fastcomments.com`。如果您的数据托管在欧盟区域，请将其更改为 EU CDN URL。