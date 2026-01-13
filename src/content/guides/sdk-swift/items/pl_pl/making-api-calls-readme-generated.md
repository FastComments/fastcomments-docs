SDK dla Swifta używa nowoczesnej składni async/await dla wszystkich wywołań API:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```