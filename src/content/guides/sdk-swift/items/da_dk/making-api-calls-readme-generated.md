Swift SDK'et bruger moderne async/await-syntaks til alle API-opkald:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```