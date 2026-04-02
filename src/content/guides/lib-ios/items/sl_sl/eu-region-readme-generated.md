---
Za uporabo podatkovnega centra v EU nastavite polje `region` v svoji konfiguraciji:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

S tem se vse API zahteve in WebSocket povezave preusmerijo na `eu.fastcomments.com`.

---
---