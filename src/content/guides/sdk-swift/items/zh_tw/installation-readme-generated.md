### Swift 套件管理器

將以下內容加入你的 `Package.swift` 檔案：

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

或在 Xcode：
1. 檔案 > 加入套件...
2. 輸入儲存庫 URL： `https://github.com/fastcomments/fastcomments-swift.git`
3. 選擇你要使用的版本

### 系統需求

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+