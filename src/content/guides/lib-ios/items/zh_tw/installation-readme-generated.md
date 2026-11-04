使用 Swift Package Manager 將 FastCommentsUI 加入您的專案。

在 Xcode 中：**File > Add Package Dependencies**，然後輸入儲存庫 URL。

或將其加入您的 `Package.swift`：

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

然後將該產品加入您的 target：

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

在需要的地方匯入兩個模組：

```swift
import FastCommentsUI
import FastCommentsSwift
```