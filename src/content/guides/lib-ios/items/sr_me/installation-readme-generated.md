Dodajte FastCommentsUI u svoj projekat koristeći Swift Package Manager.

U Xcode-u: **File > Add Package Dependencies**, zatim unesite URL repozitorijuma.

Ili ga dodajte u vaš `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Zatim dodajte product u svoj target:

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