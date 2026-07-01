---
Swift SDK използва съвременен синтаксис async/await за всички API повиквания:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---