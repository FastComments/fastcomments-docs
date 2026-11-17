Добавете FastCommentsUI към вашия проект чрез Swift Package Manager.

В Xcode: **File > Add Package Dependencies**, след което въведете URL адреса на хранилището.

Или го добавете към вашия `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

След това добавете продукта към вашия target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Импортирайте двата модула, където е необходимо:

```swift
import FastCommentsUI
import FastCommentsSwift
```