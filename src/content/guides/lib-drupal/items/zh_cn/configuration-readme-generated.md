导航到 **管理 > 配置 > 内容 > FastComments** (`/admin/config/content/fastcomments`)。

### 设置

- **租户 ID**（必填） - 您的 FastComments 租户 ID。可在 [设置 > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）下找到。
- **API 密钥** - Secure SSO、webhook 验证和页面同步所需。在 [设置 > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）下可找到。
- **单点登录模式** - 单点登录集成：
  - **无** - 无 SSO，用户以访客身份评论或创建 FastComments 帐户。
  - **简单** - 将 Drupal 用户信息（姓名、电子邮件、头像）传递给 FastComments，但不进行服务器端验证。
  - **安全** - 使用 HMAC-SHA256 验证来安全地将 Drupal 用户与 FastComments 进行身份验证（推荐）。
- **评论样式** - 要显示的小部件类型：
  - **实时评论** - 实时线程化评论。
  - **流式聊天** - 实时聊天界面。
  - **协作聊天** - 在主内容区域进行协作文本选择注释。
  - **协作聊天 + 评论** - 同时显示协作聊天和标准评论。
- **CDN URL** - FastComments CDN URL（默认：`https://cdn.fastcomments.com`）。
- **站点 URL** - FastComments 站点 URL（默认：`https://fastcomments.com`）。
- **电子邮件通知** - 当某内容上有新评论发布时，向内容作者发送电子邮件。

### 将评论添加到内容类型

通过 **结构 > 内容类型 > [类型] > 管理字段** 将 **FastComments** 字段添加到您的内容类型。该字段具有状态切换，并且每个实体可选的自定义标识符。

### 欧盟数据驻留

若需欧盟数据驻留，请更新：
- **CDN URL** 为 `https://cdn-eu.fastcomments.com`
- **站点 URL** 为 `https://eu.fastcomments.com`