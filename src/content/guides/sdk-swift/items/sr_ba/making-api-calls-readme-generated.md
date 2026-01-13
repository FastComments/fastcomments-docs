Swift SDK користи модерну async/await синтаксу за све API позиве:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```