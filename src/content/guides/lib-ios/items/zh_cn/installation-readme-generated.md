添加 **FastCommentsUI** 到您的项目，使用 Swift 包管理器。

在 Xcode 中：**File > Add Package Dependencies**，然后输入仓库 URL。

或将其添加到您的 `Package.swift`：

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

然后将产品添加到您的目标：

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

在需要的地方导入两个模块：

```swift
import FastCommentsUI
import FastCommentsSwift
```