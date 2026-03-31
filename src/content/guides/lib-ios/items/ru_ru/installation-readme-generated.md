Добавьте FastCommentsUI в ваш проект с помощью Swift Package Manager.

В Xcode: **Файл > Добавить зависимости пакета**, затем введите URL репозитория.

Или добавьте его в ваш `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Затем добавьте продукт в ваш таргет:

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