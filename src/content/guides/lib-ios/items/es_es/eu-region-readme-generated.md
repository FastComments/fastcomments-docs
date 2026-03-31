---
Para usar el centro de datos de la UE, establezca el campo `region` en su configuración:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Esto enruta todas las solicitudes de API y las conexiones WebSocket a `eu.fastcomments.com`.

---
---