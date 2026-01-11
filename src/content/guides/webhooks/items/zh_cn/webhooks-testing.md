在 Webhooks 管理界面中，每种事件类型 (Create, Update, Delete) 都有 `Send Test Payload` 按钮。Create 和 Update 事件会发送一个虚拟的 WebhookComment 对象，而测试 Delete 时会发送一个只包含 ID 的虚拟请求体。

该测试将进行两次调用，以验证 "happy"（correct API Key）和 "sad"（invalid API key）场景下的响应代码。

当测试发送无效的 API key 时，您应该返回状态码 401 以使测试完全通过。如果您没有正确检查 token 的值，就会出现错误。