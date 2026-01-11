Swift SDK използва модерен синтаксис async/await за всички API повиквания:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```