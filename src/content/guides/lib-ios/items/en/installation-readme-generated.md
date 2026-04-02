Add FastCommentsUI to your project using Swift Package Manager.

In Xcode: **File > Add Package Dependencies**, then enter the repository URL.

Or add it to your `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Then add the product to your target:

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