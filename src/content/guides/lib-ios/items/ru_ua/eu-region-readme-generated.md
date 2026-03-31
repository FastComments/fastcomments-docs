---
Чтобы использовать европейский дата-центр, задайте поле `region` в вашей конфигурации:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Это направляет все API-запросы и WebSocket-подключения на `eu.fastcomments.com`.

---
---