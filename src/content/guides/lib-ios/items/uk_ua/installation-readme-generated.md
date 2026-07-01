Add FastCommentsUI до вашого проєкту, використовуючи Swift Package Manager.

У Xcode: **File > Add Package Dependencies**, потім введіть URL репозиторію.

Або додайте його до вашого `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

Потім додайте продукт до вашої цілі:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Імпортуйте обидва модулі там, де потрібно:

```swift
import FastCommentsUI
import FastCommentsSwift
```