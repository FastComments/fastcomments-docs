---
Um das EU-Rechenzentrum zu verwenden, setzen Sie das Feld `region` in Ihrer Konfiguration:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Dies leitet alle API-Anfragen und WebSocket-Verbindungen an `eu.fastcomments.com` weiter.

---
---