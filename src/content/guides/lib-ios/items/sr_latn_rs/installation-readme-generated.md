Dodajte FastCommentsUI u vaš projekat koristeći Swift Package Manager.

U Xcode-u: **File > Add Package Dependencies**, zatim unesite URL repozitorijuma.

Ili ga dodajte u vaš `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

Zatim dodajte proizvod u vaš target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Uvezite oba modula gde je potrebno:

```swift
import FastCommentsUI
import FastCommentsSwift
```