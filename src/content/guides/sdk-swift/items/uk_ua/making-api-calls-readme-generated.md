Swift SDK використовує сучасний синтаксис async/await для всіх викликів API:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```