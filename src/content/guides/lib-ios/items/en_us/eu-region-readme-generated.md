To use the EU data center, set the `region` field in your config:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

This routes all API requests and WebSocket connections to `eu.fastcomments.com`.

---