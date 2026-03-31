使用 Swift Package Manager 將 FastCommentsUI 新增到您的專案。

在 Xcode 中：**File > Add Package Dependencies**，然後輸入儲存庫 URL。

或者將它新增到您的 `Package.swift`：

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

接著將該產品加入到您的 target：

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

在需要的地方匯入這兩個模組：

```swift
import FastCommentsUI
import FastCommentsSwift
```