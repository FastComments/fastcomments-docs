使用 Swift 包管理器将 FastCommentsUI 添加到您的项目。

在 Xcode 中：**File > Add Package Dependencies**，然后输入仓库 URL。

或者将其添加到您的 `Package.swift`：

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

然后将该产品添加到您的目标：

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

在需要的地方导入这两个模块：

```swift
import FastCommentsUI
import FastCommentsSwift
```