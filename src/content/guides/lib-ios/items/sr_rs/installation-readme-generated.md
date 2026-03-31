Додајте FastCommentsUI у ваш пројекат користећи Swift Package Manager.

У Xcode: **File > Add Package Dependencies**, затим унесите URL репозиторијума.

Или додајте у ваш `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Затим додајте product у ваш target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Увезите оба модула где је потребно:

```swift
import FastCommentsUI
import FastCommentsSwift
```