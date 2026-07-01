---
Swift SDK 使用现代的 async/await 语法进行所有 API 调用：

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```
---