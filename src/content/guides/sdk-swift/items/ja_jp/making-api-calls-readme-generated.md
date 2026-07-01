Swift SDK は、すべての API 呼び出しにモダンな async/await 構文を使用します：

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```