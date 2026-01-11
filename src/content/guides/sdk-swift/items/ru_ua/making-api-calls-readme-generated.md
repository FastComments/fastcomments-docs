Swift SDK использует современный синтаксис async/await для всех вызовов API:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```