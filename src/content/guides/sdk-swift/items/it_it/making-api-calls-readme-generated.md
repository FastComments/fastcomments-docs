Lo SDK Swift utilizza la sintassi moderna async/await per tutte le chiamate API:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```