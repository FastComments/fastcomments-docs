Tilføj FastCommentsUI til dit projekt ved hjælp af Swift Package Manager.

I Xcode: **File > Add Package Dependencies**, og indtast derefter repository-URL'en.

Eller tilføj det til din `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

Tilføj derefter produktet til dit target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Importér begge moduler, hvor det er nødvendigt:

```swift
import FastCommentsUI
import FastCommentsSwift
```