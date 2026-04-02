---
Da biste koristili EU data centar, postavite polje `region` u svojoj konfiguraciji:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Ovo usmerava sve API zahteve i WebSocket konekcije na `eu.fastcomments.com`.

---
---