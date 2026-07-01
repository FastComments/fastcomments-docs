---
O SDK Swift usa a sintaxe moderna async/await para todas as chamadas de API:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---