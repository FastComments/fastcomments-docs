Swift SDK, tüm API çağrıları için modern async/await sözdizimini kullanır:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```