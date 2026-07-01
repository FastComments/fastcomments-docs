---
Swift SDK uporablja sodobno sintakso async/await za vse klice API-jev:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---