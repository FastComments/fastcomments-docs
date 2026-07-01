Add FastCommentsUI to your project using Swift Package Manager.

使用 Swift Package Manager 將 FastCommentsUI 添加到您的專案中。

In Xcode: **File > Add Package Dependencies**, then enter the repository URL.

在 Xcode 中：**File > Add Package Dependencies**，然後輸入儲存庫 URL。

Or add it to your `Package.swift`:

或將其加入您的 `Package.swift`：

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

Then add the product to your target:

接著將產品加入您的目標：

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Import both modules where needed:

在需要的地方匯入兩個模組：

```swift
import FastCommentsUI
import FastCommentsSwift
```