---
Swift SDK користи модерну async/await синтаксу за све API позиве:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---