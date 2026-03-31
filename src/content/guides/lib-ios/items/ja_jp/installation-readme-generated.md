---
Swift Package Managerを使用してFastCommentsUIをプロジェクトに追加します。

Xcodeで: **File > Add Package Dependencies** を選択し、リポジトリのURLを入力します。

または、`Package.swift` に追加します:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

次に、ターゲットに製品を追加します:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

必要な箇所で両方のモジュールをインポートします:

```swift
import FastCommentsUI
import FastCommentsSwift
```
---