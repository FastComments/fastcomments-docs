Το Swift SDK χρησιμοποιεί σύγχρονη σύνταξη async/await για όλες τις κλήσεις API:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```