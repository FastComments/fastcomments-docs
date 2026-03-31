Додајте FastCommentsUI у ваш пројекат користећи Swift Package Manager.

У Xcode-у: **File > Add Package Dependencies**, затим унесите repository URL.

Или га додајте у ваш `Package.swift`:

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

Импортујте оба модула где је потребно:

```swift
import FastCommentsUI
import FastCommentsSwift
```