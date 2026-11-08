Swift SDK सभी API कॉल के लिए आधुनिक async/await सिंटैक्स का उपयोग करता है:

```swift
let response = try await PublicAPI.getCommentsPublic(
    tenantId: "your-tenant-id",
    urlId: "page-url-id"
)
```