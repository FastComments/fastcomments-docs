Add FastCommentsUI v svoj projekt z uporabo Swift Package Managerja.

V Xcode: **File > Add Package Dependencies**, nato vnesite URL repozitorija.

Ali ga dodajte v svoj `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

Nato dodajte izdelek v svoj cilj:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Uvozite oba modula, kjer je potrebno:

```swift
import FastCommentsUI
import FastCommentsSwift
```