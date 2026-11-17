Додајте FastCommentsUI у ваш пројекат користећи Swift Package Manager.

У Xcode-у: **File > Add Package Dependencies**, затим унесите URL репозиторијума.

Или га додајте у ваш `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

Затим додајте производ у ваш target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Увезите оба модула тамо где је потребно:

```swift
import FastCommentsUI
import FastCommentsSwift
```