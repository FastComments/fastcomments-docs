Add FastCommentsUI'yi projenize Swift Package Manager kullanarak ekleyin.

In Xcode: **File > Add Package Dependencies**, ardından depo URL'sini girin.

Or add it to your `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
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