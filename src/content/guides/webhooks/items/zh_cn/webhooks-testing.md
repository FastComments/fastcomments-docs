新的和编辑 webhook 页面有一个 `Send Test Payload` 按钮，它会向表单中当前填写的 URL 发送请求，无论该 URL 是否已保存。Create 和 Update 事件会发送一个虚拟的 WebhookComment 对象，而测试 Delete 则会发送一个仅包含 ID 的虚拟请求体。

## 验证载荷

在测试你的 webhook 集成时，请验证传入的请求是否包含以下头部：

1. **`X-FastComments-Timestamp`** - Unix 时间戳（秒）
2. **`X-FastComments-Signature`** - HMAC-SHA256 签名

在签名方案引入之前创建的 webhook 还会收到一个包含你的 API Secret 的 **`token`** 头部。新的 webhook 则不会。

使用 HMAC 签名验证来确保载荷的真实性。

## 测试工具

在开发过程中，你可以使用 [webhook.site](https://webhook.site) 或 [ngrok](https://ngrok.com) 等工具来检查传入的 webhook 载荷。

## 事件类型

- **Create 事件**：在创建新评论时触发。
- **Update 事件**：在编辑评论时触发。
- **Delete 事件**：在删除评论时触发。

每个 webhook 绑定到一个事件和一种 HTTP 方法（POST、PUT 或 DELETE）。每个事件都会在请求体中包含完整的评论数据（载荷格式请参见[数据结构](/guide-webhooks.html#webhooks-structures)）。