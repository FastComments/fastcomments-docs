### Swift 包管理器

在你的 `Package.swift` 文件中添加以下内容：

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.0.0")
]
```

或者在 Xcode 中：
1. 文件 > 添加包...
2. 输入仓库 URL： `https://github.com/fastcomments/fastcomments-swift.git`
3. 选择你想使用的版本

### 要求

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+