---
SDK Swift używa nowoczesnej składni async/await we wszystkich wywołaniach API:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---