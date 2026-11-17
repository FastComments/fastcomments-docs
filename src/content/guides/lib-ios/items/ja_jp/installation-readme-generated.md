Swift Package Managerを使用して、FastCommentsUIをプロジェクトに追加します。

Xcodeで: **File > Add Package Dependencies**を選択し、リポジトリのURLを入力します。

または、`Package.swift`に追加します:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

次に、ターゲットにプロダクトを追加します:

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