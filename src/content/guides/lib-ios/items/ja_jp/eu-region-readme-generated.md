---
EU データセンターを使用するには、設定の `region` フィールドを設定してください:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

これにより、すべての API リクエストと WebSocket 接続が `eu.fastcomments.com` にルーティングされます。

---
---