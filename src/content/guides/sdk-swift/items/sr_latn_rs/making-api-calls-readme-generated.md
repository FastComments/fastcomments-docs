---
Swift SDK koristi modernu async/await sintaksu za sve API pozive:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---