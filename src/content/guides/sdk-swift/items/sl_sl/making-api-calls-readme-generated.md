Swift SDK uporablja sodobno sintakso async/await za vse klice API-ja:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```