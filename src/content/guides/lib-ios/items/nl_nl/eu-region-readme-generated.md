---
Om het EU-datacenter te gebruiken, stel het `region`-veld in uw config in:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

Dit leidt alle API-verzoeken en WebSocket-verbindingen naar `eu.fastcomments.com`.

---
---