---
Щоб використовувати європейський центр обробки даних, задайте поле `region` у вашій конфігурації:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Це спрямовує всі запити API та з'єднання WebSocket до `eu.fastcomments.com`.

---
---