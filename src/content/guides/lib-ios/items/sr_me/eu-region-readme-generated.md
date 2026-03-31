---
Да бисте користили ЕУ центар података, подесите поље `region` у вашем config:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Овим се усмјеравају сви API захтјеви и WebSocket везе на `eu.fastcomments.com`.

---
---