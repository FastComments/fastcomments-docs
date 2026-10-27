All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## 必填

- **Tenant ID** - 您的 FastComments 租户 ID。可在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）下找到。
- **API Secret** - 用于安全 SSO、Webhook 验证和页面同步。可在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）下找到。

## 存储 API Secret

The module reads the API Secret from the first of these sources that has a value:

1. **Key module** - 安装 [Key](https://www.drupal.org/project/key)，创建一个 *Authentication* 密钥，并在设置表单的 **API Secret key** 中选择它。该密钥可以由环境变量或网站根目录之外的文件提供。
2. **settings.php** - 将 `$config['fastcomments.settings']['api_secret'] = 'your-secret';` 添加到 `settings.php` 或 `settings.local.php`。该密钥永不进入导出的配置。
3. **设置表单** - 在 **API Secret** 字段中输入。此值会存储在 `fastcomments.settings` 中，从而出现在配置导出中。当此为活动来源时，Drupal 状态报告会给出警告。

The form shows which source is active. Leaving the field blank keeps the stored secret. Tick **Remove the API Secret stored in configuration** to delete it after moving the secret to the Key module or `settings.php`.

## 评论样式

Pick the widget that matches how you want people to talk on your site.

- **Live Comments** - 实时线程式评论。
- **Streaming Chat** - 实时聊天界面，适用于活动和直播。
- **Collab Chat** - 在主要内容区域进行文本选择注释。访客可高亮文本并在上下文中发起讨论。
- **Collab Chat + Comments** - 在同一页面上同时提供协作聊天和标准评论。

## SSO 模式

- **None** - 不使用 SSO。用户以访客身份评论或创建 FastComments 账户。
- **Simple** - 将 Drupal 用户信息（姓名、电子邮件、头像）传递给 FastComments，且不进行服务器端验证。
- **Secure** - 使用 HMAC-SHA256 验证 Drupal 用户与 FastComments 的对应关系。建议在已配置 API Secret 时使用。

See the `Single Sign-On (SSO)` section for details.

## 其他设置

- **CDN URL** - 默认值为 `https://cdn.fastcomments.com`。
- **Site URL** - 默认值为 `https://fastcomments.com`。
- **Email notifications** - 当有新评论发布在其内容上时，向内容作者发送电子邮件。

For EU data residency, see the `EU Data Residency` section.