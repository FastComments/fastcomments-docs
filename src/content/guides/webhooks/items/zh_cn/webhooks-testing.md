在 Webhooks 管理页面中，每个事件类型（Create、Update、Delete）都有 `Send Test Payload` 按钮。Create 和 Update 事件会发送一个虚拟的 WebhookComment 对象，而测试 Delete 时会发送仅包含 ID 的虚拟请求体。

## 验证有效载荷

测试您的 webhook 集成时，请确认传入请求包含以下头部：

1. **`token`** - 您的 API Secret  
2. **`X-FastComments-Timestamp`** - Unix 时间戳（秒）  
3. **`X-FastComments-Signature`** - HMAC-SHA256 签名

使用 HMAC 签名验证以确保有效载荷的真实性。

## 测试工具

开发过程中可以使用诸如 [webhook.site](https://webhook.site) 或 [ngrok](https://ngrok.com) 之类的工具来检查传入的 webhook 有效载荷。

## 事件类型

- **Create Event**: 当创建新评论时触发。默认方法：PUT  
- **Update Event**: 当编辑评论时触发。默认方法：PUT  
- **Delete Event**: 当删除评论时触发。默认方法：DELETE

每个事件在请求体中包含完整的评论数据（有关有效载荷格式，请参见 [数据结构](/guides/webhooks/webhooks-structures)）。