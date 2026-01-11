Le SDK Swift utilise la syntaxe moderne async/await pour tous les appels d'API :

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```