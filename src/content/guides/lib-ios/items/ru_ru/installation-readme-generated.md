Добавьте FastCommentsUI в ваш проект с помощью Swift Package Manager.

В Xcode: **File > Add Package Dependencies**, затем введите URL репозитория.

Или добавьте его в ваш `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

Затем добавьте продукт в вашу цель:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Импортируйте оба модуля там, где это необходимо:

```swift
import FastCommentsUI
import FastCommentsSwift
```