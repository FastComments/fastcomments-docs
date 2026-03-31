Dodajte FastCommentsUI u svoj projekt koristeći Swift Package Manager.

U Xcodeu: **File > Add Package Dependencies**, zatim unesite URL repozitorija.

Ili ga dodajte u svoj `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Zatim dodajte proizvod vašem targetu:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Uvezite oba modula gdje je potrebno:

```swift
import FastCommentsUI
import FastCommentsSwift
```