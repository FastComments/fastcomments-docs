---
要使用欧盟数据中心，请在配置中设置 `region` 字段：

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

这会将所有 API 请求和 WebSocket 连接路由到 `eu.fastcomments.com`。

---
---