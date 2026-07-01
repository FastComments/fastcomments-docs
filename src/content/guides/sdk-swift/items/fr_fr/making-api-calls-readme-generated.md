---
Le SDK Swift utilise la syntaxe async/await moderne pour tous les appels d'API :

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---