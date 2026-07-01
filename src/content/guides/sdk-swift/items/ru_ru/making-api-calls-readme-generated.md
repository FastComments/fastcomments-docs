Swift SDK использует современный синтаксис async/await для всех вызовов API:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```