Fügen Sie FastCommentsUI mit dem Swift Package Manager zu Ihrem Projekt hinzu.

In Xcode: **File > Add Package Dependencies**, und geben Sie dann die Repository-URL ein.

Oder fügen Sie es zu Ihrer `Package.swift` hinzu:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

Fügen Sie dann das Produkt zu Ihrem Target hinzu:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Importieren Sie beide Module, wo benötigt:

```swift
import FastCommentsUI
import FastCommentsSwift
```