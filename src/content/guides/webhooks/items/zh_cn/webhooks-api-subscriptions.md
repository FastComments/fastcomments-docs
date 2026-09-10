Webhooks 也可以通过 REST API 进行管理。这就是像 Zapier 这样的集成在不触及仪表板的情况下订阅评论事件的方式，并且它遵循 REST Hooks 模式：订阅、接收事件、取消订阅。

API 订阅与仪表板中配置的 webhook 并存。评论事件会发送给每个匹配其域的 webhook，每个 webhook 都作为单独的投递，无论 webhook 是以何种方式创建的。

## 身份验证

每个请求都需要在 `x-api-key` 头部（或 `API_KEY` 查询参数）中提供您的 API Key，并在 `tenantId` 查询参数中提供租户 ID。这两项信息均显示在仪表板的 API Secret 页面上。

## 订阅

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| 字段 | 必需 | 描述 |
|-------|----------|-------------|
| `url` | 是 | 绝对的 http 或 https URL。 |
| `event` | 是 | `comment-created`、`comment-updated` 或 `comment-deleted`。 |
| `domain` | 否 | 您账户配置中的域。默认是 `*`，会接收所有域的事件。 |
| `method` | 否 | `POST`（默认）、`PUT` 或 `DELETE`。 |

响应中包含订阅信息：

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

再次使用相同的 URL 订阅相同的事件和域时，会返回已有的订阅，而不是创建重复的订阅，从而客户端可以安全地重试。每个租户最多可拥有 50 个 API 订阅。

## 列表

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

返回该租户的所有 webhook，包括在仪表板中管理的（`"source": "dashboard"`）。可使用 `event`、`domain` 或 `source` 进行过滤。

## 取消订阅

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

删除订阅时也会丢弃仍在队列中的任何事件。只有通过 API 创建的订阅才能以此方式删除。仪表板中的 webhook 需在 Webhooks 页面进行编辑。

## 负载和签名

投递使用与仪表板 webhook 相同的负载（参见数据结构），并使用相同的 HMAC 方案进行签名（参见安全性与 API 令牌）。API 订阅永不接收旧版的 `token` 头部，请改为验证 `X-FastComments-Signature` 头部。

## 示例负载

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

返回账户最近的评论，结构完全与投递的负载相同，便于集成在首个事件到达前展示真实的示例数据。`event` 为可选参数，仅用于验证，因为每个事件都传递相同的评论对象。`limit` 默认值为 3，取值范围为 1 到 10。消耗 2 个 API 积分。

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## 对 410 Gone 的响应

如果 API 订阅的端点返回 HTTP `410 Gone`，FastComments 会将其视为取消订阅：该订阅以及其排队的事件将被删除，不再尝试后续投递。仪表板中配置的 webhook 永不会被自动删除；对它们而言，410 只是一次普通的失败。其他任何失败状态都会被重试，最终会禁用该 webhook，详见“工作原理与重试处理”。

## 仪表板

API 订阅会出现在 Webhooks 列表中，来源标记为 **API**，管理员可以对其进行编辑、禁用、重新启用或删除。

---