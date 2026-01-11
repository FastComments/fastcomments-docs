Swift SDK는 모든 API 호출에 대해 최신 async/await 구문을 사용합니다:

```swift
let response = try await publicApi.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```