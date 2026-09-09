---
FastComments 仅支持对 Comment 资源的 webhook。

我们支持对评论的创建、删除和更新的 webhook。

我们系统中将这些视为独立的事件，因此具有不同的语义
并且 webhook 事件的结构也不同。

任意数量的端点都可以订阅同一事件：每个域名可以在
仪表板中配置一个 webhook，并且可以通过 API 创建更多订阅（参见通过 API 管理 Webhook）。

---