The Swift SDK uses modern async/await syntax for all API calls:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```