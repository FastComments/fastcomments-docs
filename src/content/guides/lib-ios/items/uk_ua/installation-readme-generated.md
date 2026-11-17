Додайте FastCommentsUI до вашого проєкту за допомогою Swift Package Manager.

У Xcode: **File > Add Package Dependencies**, потім введіть URL репозиторію.

Або додайте його до вашого `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

Потім додайте продукт до вашої цілі (target):

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Імпортуйте обидва модулі там, де це потрібно:

```swift
import FastCommentsUI
import FastCommentsSwift
```