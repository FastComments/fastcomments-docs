---
Swift パッケージマネージャーを使用して FastCommentsUI をプロジェクトに追加します。

Xcode では: **File > Add Package Dependencies**、その後リポジトリ URL を入力します。

または、`Package.swift` に追加します:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

次に、製品をターゲットに追加します:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

必要な場所で両方のモジュールをインポートします:

```swift
import FastCommentsUI
import FastCommentsSwift
```
---