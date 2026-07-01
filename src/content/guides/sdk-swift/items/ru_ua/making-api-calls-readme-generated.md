---
Swift SDK використовує сучасний синтаксис async/await для всіх API викликів:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---