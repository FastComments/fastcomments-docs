Das Swift SDK verwendet die moderne async/await-Syntax für alle API-Aufrufe:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```