---
Da biste koristili podatkovni centar EU, postavite polje `region` u vašoj konfiguraciji:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Ovo usmjerava sve API zahtjeve i WebSocket veze na `eu.fastcomments.com`.

---
---