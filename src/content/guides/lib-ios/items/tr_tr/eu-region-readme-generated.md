---
AB veri merkezini kullanmak için, yapılandırmanızda `region` alanını şu şekilde ayarlayın:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Bu, tüm API isteklerini ve WebSocket bağlantılarını `eu.fastcomments.com` adresine yönlendirir.

---
---