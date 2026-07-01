Add FastCommentsUI til dit projekt ved hjælp af Swift Package Manager.

I Xcode: **File > Add Package Dependencies**, indtast derefter repository‑URL'en.

Eller tilføj den til din `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

Tilføj derefter produktet til din target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Importer begge moduler, hvor de er nødvendige:

```swift
import FastCommentsUI
import FastCommentsSwift
```