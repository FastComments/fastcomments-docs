Swift SDK'en bruger moderne async/await-syntaks til alle API‑kald:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```