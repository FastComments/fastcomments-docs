Dodajte FastCommentsUI u vaš projekat koristeći Swift Package Manager.

U Xcode-u: **Datoteka > Dodaj zavisnosti paketa**, zatim unesite URL repozitorijuma.

Ili dodajte ga u vaš `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Zatim dodajte product u vaš target:

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