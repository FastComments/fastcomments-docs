插件设置页面位于 **Site Administration > Plugins > Local plugins > FastComments**。可用选项包括：

#### Tenant ID

您的 FastComments 租户 ID。可在 <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments 仪表板</a> 的账户设置中找到。

#### API 密钥

您的 API Secret 密钥，在 Secure SSO 模式下必需。可在 <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">我的账户 > API 密钥</a> 找到。

#### SSO 模式

选择用户的身份验证方式。有关每个选项的详细信息，请参见 [SSO 模式](#moodle-sso-modes) 部分。

- **Secure** (recommended) - 服务器端 HMAC-SHA256 签名认证
- **Simple** - 客户端用户数据，无签名
- **None** - 匿名评论，不与 Moodle 登录集成

#### 页面上下文

控制评论显示的位置：

- **Course pages** - 在课程主页面显示评论
- **Module/activity pages** - 在单个活动和资源页面显示评论
- **Both** - 在所有页面类型显示评论

#### 评论样式

选择评论体验。有关每种模式的屏幕截图，请参见 [评论样式](#moodle-commenting-styles)。

- **Comments** - 标准线程式评论小部件，显示在页面内容下方
- **Collab Chat** - 内联文本选择讨论，带在线状态指示
- **Both** - 同时启用评论和协作聊天

#### CDN URL

FastComments 的 CDN URL。默认值为 `https://cdn.fastcomments.com`。如果您的数据托管在欧盟区域，请将其更改为欧盟 CDN 的 URL。

---