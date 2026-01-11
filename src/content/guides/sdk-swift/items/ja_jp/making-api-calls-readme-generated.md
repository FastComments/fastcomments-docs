Swift SDK はすべての API 呼び出しに対してモダンな async/await 構文を使用します:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```