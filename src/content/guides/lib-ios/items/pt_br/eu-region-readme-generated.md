---
Para usar o datacenter da UE, defina o campo `region` na sua configuração:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Isso direciona todas as requisições de API e conexões WebSocket para `eu.fastcomments.com`.

---
---