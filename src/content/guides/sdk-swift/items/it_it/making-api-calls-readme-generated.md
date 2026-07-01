---
L'SDK Swift utilizza la moderna sintassi async/await per tutte le chiamate API:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---