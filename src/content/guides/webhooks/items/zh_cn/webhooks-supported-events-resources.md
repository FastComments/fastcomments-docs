---
FastComments 仅支持对 Comment 资源的 webhook。

我们支持对评论的创建、删除和更新的 webhook。

这些在我们的系统中被视为独立的事件，因此 webhook 事件具有不同的语义和结构。

任意数量的端点可以从仪表板或通过 API 订阅同一事件（参见通过 API 管理 Webhook）。每个 webhook 都会独立发送。

---