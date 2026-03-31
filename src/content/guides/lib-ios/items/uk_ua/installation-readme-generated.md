Додайте FastCommentsUI у свій проєкт, використовуючи Swift Package Manager.

У Xcode: **File > Add Package Dependencies**, потім введіть URL репозиторію.

Або додайте його до вашого `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Потім додайте продукт до вашого target:

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