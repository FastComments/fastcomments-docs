### Swift 套件管理器

將下列內容新增到您的 `Package.swift` 檔案：

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "2.0.0")
]
```

或在 Xcode 中：
1. 檔案 > 新增套件...
2. 輸入儲存庫 URL：`https://github.com/fastcomments/fastcomments-swift.git`
3. 選擇您要使用的版本

### 系統需求

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+