---
For at bruge EU-datacenteret skal du sætte feltet `region` i din konfiguration:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Dette dirigerer alle API-forespørgsler og WebSocket-forbindelser til `eu.fastcomments.com`.

---
---