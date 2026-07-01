---
ה‑SDK של Swift משתמש בתחביר async/await מודרני לכל קריאות ה‑API:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---