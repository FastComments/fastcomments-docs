Dodajte FastCommentsUI v svoj projekt z uporabo Swift Package Manager.

V Xcode: **File > Add Package Dependencies**, nato vnesite URL repozitorija.

Ali pa ga dodajte v svoj `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

Nato dodajte produkt svojemu tarči:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Uvozite oba modula, kjer je to potrebno:

```swift
import FastCommentsUI
import FastCommentsSwift
```