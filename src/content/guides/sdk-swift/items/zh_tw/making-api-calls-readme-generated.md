---
Swift SDK 使用現代的 async/await 語法來呼叫所有 API：

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---