Το Swift SDK χρησιμοποιεί τη σύγχρονη σύνταξη async/await για όλες τις κλήσεις API:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```