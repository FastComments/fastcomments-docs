Swift SDK 對所有 API 呼叫使用現代的 async/await 語法：

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```