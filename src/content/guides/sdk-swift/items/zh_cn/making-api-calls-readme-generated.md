Swift SDK 为所有 API 调用使用现代的 async/await 语法：

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```