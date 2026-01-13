ה-Swift SDK משתמש בתחביר async/await מודרני בכל קריאות ה-API:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```