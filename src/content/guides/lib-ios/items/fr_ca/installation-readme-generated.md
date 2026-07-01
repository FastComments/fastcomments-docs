Add FastCommentsUI à votre projet en utilisant Swift Package Manager.

In Xcode: **File > Add Package Dependencies**, then enter the repository URL.

Or add it to votre `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

Then add the product à votre target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Import both modules where needed:

```swift
import FastCommentsUI
import FastCommentsSwift
```