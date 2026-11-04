插件设置页面位于 **Site Administration > Plugins > Local plugins > FastComments**。可用选项如下：

#### Tenant ID

您的 FastComments Tenant ID。可在 <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments 控制面板</a>的账户设置中找到。

#### API Secret

您的 API Secret 密钥，安全 SSO 模式下必需。可在 <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a> 找到。

#### SSO Mode

选择用户身份验证方式。有关各选项的详细信息，请参阅 [SSO 模式](#moodle-sso-modes) 部分。

- **Secure**（推荐）- 服务器端 HMAC-SHA256 签名身份验证
- **Simple** - 客户端用户数据，无签名
- **None** - 匿名评论，不集成 Moodle 登录

#### Page Contexts

控制评论显示的位置：

- **Course pages** - 课程主页面上的评论
- **Module/activity pages** - 各个活动和资源页面上的评论
- **Both** - 所有页面类型上均显示评论

#### Commenting Style

选择评论体验。有关各模式的截图，请参阅 [评论样式](#moodle-commenting-styles)。

- **Comments** - 页面内容下方的标准线程评论组件
- **Collab Chat** - 内联文本选择讨论，带在线状态指示器
- **Both** - 评论和协作聊天同时启用

#### CDN URL

FastComments CDN URL。默认为 `https://cdn.fastcomments.com`。如果您的数据托管在欧盟区域，请将其更改为欧盟 CDN URL。