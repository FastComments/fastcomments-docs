---
За да използвате центъра за данни в ЕС, задайте полето `region` във вашата конфигурация:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Това насочва всички API заявки и WebSocket връзки към `eu.fastcomments.com`.

---
---