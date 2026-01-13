Swift SDK koristi modernu sintaksu async/await za sve API pozive:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```