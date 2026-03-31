---
Voeg FastCommentsUI toe aan je project met Swift Package Manager.

In Xcode: **Bestand > Voeg pakketafhankelijkheden toe**, voer vervolgens de repository-URL in.

Of voeg het toe aan je `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Voeg vervolgens het product toe aan je target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Importeer beide modules waar nodig:

```swift
import FastCommentsUI
import FastCommentsSwift
```
---