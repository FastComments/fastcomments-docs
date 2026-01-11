De Swift SDK gebruikt moderne async/await-syntaxis voor alle API-aanroepen:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```