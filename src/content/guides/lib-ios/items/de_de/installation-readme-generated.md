Fügen Sie FastCommentsUI Ihrem Projekt mit dem Swift Package Manager hinzu.

In Xcode: **Datei > Paketabhängigkeiten hinzufügen**, dann die Repository-URL eingeben.

Oder fügen Sie es zu Ihrer `Package.swift` hinzu:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
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

Importieren Sie beide Module dort, wo sie benötigt werden:

```swift
import FastCommentsUI
import FastCommentsSwift
```