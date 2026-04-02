Добавете FastCommentsUI към вашия проект, като използвате Swift Package Manager.

В Xcode: **File > Add Package Dependencies**, след това въведете URL адреса на репозиторията.

Или го добавете във вашия `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

След това добавете продукта към вашата цел:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Импортирайте и двата модула, където е необходимо:

```swift
import FastCommentsUI
import FastCommentsSwift
```