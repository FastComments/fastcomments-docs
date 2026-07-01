---
Das Swift SDK verwendet moderne async/await‑Syntax für alle API‑Aufrufe:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---