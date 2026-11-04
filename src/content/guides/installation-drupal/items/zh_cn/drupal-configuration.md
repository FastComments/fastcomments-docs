所有设置均位于 `Administration > Configuration > Content > FastComments`（`/admin/config/content/fastcomments`）。

## 必填项

- **Tenant ID** - 您的 FastComments Tenant ID。可在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）下找到。
- **API Secret** - 安全 SSO、Webhook 验证和页面同步所必需。可在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）下找到。

## 存储 API Secret

模块会从以下来源中读取第一个有值的 API Secret：

1. **Key 模块** - 安装 [Key](https://www.drupal.org/project/key)，创建一个 *Authentication* 密钥，并在设置表单的 **API Secret key** 下选择它。该密钥可以由环境变量或 Web 根目录之外的文件支持。
2. **settings.php** - 在 `settings.php` 或 `settings.local.php` 中添加 `$config['fastcomments.settings']['api_secret'] = 'your-secret';`。该密钥永远不会进入导出的配置中。
3. **设置表单** - 在 **API Secret** 字段中输入。这会将值存储在 `fastcomments.settings` 中，因此它会出现在配置导出中。当这是当前生效的来源时，Drupal 状态报告会发出警告。

表单会显示当前生效的来源。将该字段留空可保留已存储的密钥。勾选 **Remove the API Secret stored in configuration** 可在将密钥迁移到 Key 模块或 `settings.php` 后删除它。

## 评论样式

选择与您希望人们在您网站上交流方式相匹配的组件。

- **Live Comments** - 实时线程化评论。
- **Streaming Chat** - 实时聊天界面，适合活动和直播。
- **Collab Chat** - 在主要内容区域进行文本选择标注。访客可以高亮文本并在上下文中发起讨论。
- **Collab Chat + Comments** - 在同一页面上同时提供协作聊天和标准评论。

## SSO 模式

- **None** - 无 SSO。用户以访客身份评论或创建 FastComments 账户。
- **Simple** - 将 Drupal 用户信息（姓名、邮箱、头像）传递给 FastComments，无需服务器端验证。
- **Secure** - 使用 HMAC-SHA256 向 FastComments 验证 Drupal 用户。在已配置 API Secret 的情况下推荐使用。

详情请参阅 `Single Sign-On (SSO)` 部分。

## 其他设置

- **CDN URL** - 默认为 `https://cdn.fastcomments.com`。
- **Site URL** - 默认为 `https://fastcomments.com`。
- **Email notifications** - 当有新评论发布在内容作者的内容上时，向其发送电子邮件。

关于欧盟数据驻留，请参阅 `EU Data Residency` 部分。