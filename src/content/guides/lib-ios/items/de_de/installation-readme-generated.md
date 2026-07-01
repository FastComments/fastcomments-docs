Add FastCommentsUI zu Ihrem Projekt mit dem Swift Package Manager.

In Xcode: **File > Add Package Dependencies**, geben Sie dann die Repository-URL ein.

Oder fügen Sie es zu Ihrer `Package.swift` hinzu:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

Fügen Sie dann das Produkt zu Ihrem Ziel hinzu:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Importieren Sie beide Module dort, wo sie benötigt werden:

```swift
import FastCommentsUI
import FastCommentsSwift
```