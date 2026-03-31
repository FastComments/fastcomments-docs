---
Aby korzystać z centrum danych UE, ustaw pole `region` w swojej konfiguracji:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Spowoduje to przekierowanie wszystkich żądań API i połączeń WebSocket do `eu.fastcomments.com`.

---
---