---
El SDK de Swift utiliza la sintaxis moderna async/await para todas las llamadas a la API:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---