---
Да бисте користили ЕУ центар података, подесите поље `region` у вашој конфигурацији:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Ово усмерава све API захтеве и WebSocket везе на `eu.fastcomments.com`.

---
---