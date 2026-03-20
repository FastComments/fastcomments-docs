In the Webhooks admin there are `Send Test Payload` buttons for each event type (Create, Update, Delete). The Create and Update events send a dummy WebhookComment object, while testing Delete will send a dummy request body with just an ID.

## 验证有效载荷

在测试您的 webhook 集成时，请确认传入请求包含以下头：

1. **`token`** - 您的 API 密钥
2. **`X-FastComments-Timestamp`** - Unix 时间戳（秒）
3. **`X-FastComments-Signature`** - HMAC-SHA256 签名

使用 HMAC 签名验证以确保有效载荷的真实性。

## 测试工具

您可以使用像 [webhook.site](https://webhook.site) 或 [ngrok](https://ngrok.com) 这样的工具在开发期间检查传入的 webhook 有效载荷。

## 事件类型

- **Create Event**: 当创建新评论时触发。默认方法：PUT
- **Update Event**: 当编辑评论时触发。默认方法：PUT
- **Delete Event**: 当删除评论时触发。默认方法：DELETE

每个事件在请求主体中包含完整的评论数据（有关有效载荷格式，请参见 [数据结构](/guide-webhooks.html#webhooks-structures)）。