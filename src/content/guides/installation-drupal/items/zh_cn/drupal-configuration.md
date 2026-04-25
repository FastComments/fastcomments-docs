所有设置位于 `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`)。

## 必填项

- **Tenant ID** - 您的 FastComments Tenant ID。可在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）下找到。
- **API Secret** - 在 Secure SSO、webhook 验证和页面同步中需要。可在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）下找到。

## 评论样式

选择与您希望访客在网站上交流方式相匹配的小部件。

- **Live Comments** - 实时的线程式评论。
- **Streaming Chat** - 实时聊天界面，适用于活动和直播。
- **Collab Chat** - 在主要内容区域进行文本选择注释。访客高亮文本并在上下文中开始讨论。
- **Collab Chat + Comments** - 在同一页面同时显示 Collab Chat 和标准评论。

## SSO 模式

- **None** - 没有 SSO。用户以访客身份评论或创建 FastComments 帐户。
- **Simple** - 将 Drupal 用户信息（name、email、avatar）传递给 FastComments，而无需服务器端验证。
- **Secure** - 使用 HMAC-SHA256 将 Drupal 用户与 FastComments 进行验证。当您配置了 API Secret 时推荐使用。

详情请参见 `Single Sign-On (SSO)` 部分。

## 其他设置

- **CDN URL** - 默认为 `https://cdn.fastcomments.com`。
- **Site URL** - 默认为 `https://fastcomments.com`。
- **Email notifications** - 在内容上发布新评论时，向内容作者发送电子邮件通知。

有关欧盟数据驻留，请参见 `EU Data Residency` 部分。