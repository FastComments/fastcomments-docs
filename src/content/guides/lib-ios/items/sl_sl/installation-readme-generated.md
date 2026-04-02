Dodajte FastCommentsUI v vaš projekt z uporabo Swift Package Managerja.

V Xcode: **File > Add Package Dependencies**, nato vnesite URL repozitorija.

Ali ga dodajte v vaš `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Nato dodajte produkt v vaš target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Uvozite oba modula, kjer ju potrebujete:

```swift
import FastCommentsUI
import FastCommentsSwift
```