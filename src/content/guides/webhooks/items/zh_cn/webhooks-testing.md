The new and edit webhook pages have a `Send Test Payload` button that sends a request to the URL currently in the form, whether or not it has been saved. The Create and Update events send a dummy WebhookComment object, while testing Delete will send a dummy request body with just an ID.

## 验证负载

在测试 webhook 集成时，请验证传入请求包含以下头部：

1. **`X-FastComments-Timestamp`** - Unix 时间戳（秒）
2. **`X-FastComments-Signature`** - HMAC-SHA256 签名

在引入签名方案之前创建的 webhook 仍会收到包含您 API Secret 的 **`token`** 头部。新 webhook 不会收到此头部。

使用 HMAC 签名验证来确保负载的真实性。

## 测试工具

您可以使用诸如 [webhook.site](https://webhook.site) 或 [ngrok](https://ngrok.com) 等工具，在开发期间检查传入的 webhook 负载。

## 事件类型

- **Create Event**：当创建新评论时触发。
- **Update Event**：当编辑评论时触发。
- **Delete Event**：当删除评论时触发。

每个 webhook 绑定一个事件和一种 HTTP 方法（POST、PUT 或 DELETE）。每个事件在请求体中包含完整的评论数据（请参阅 [Data Structures](/guide-webhooks.html#webhooks-structures) 了解负载格式）。

---