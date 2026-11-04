Aggiungi FastCommentsUI al tuo progetto usando Swift Package Manager.

In Xcode: **File > Add Package Dependencies**, quindi inserisci l'URL del repository.

Oppure aggiungilo al tuo `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

Poi aggiungi il prodotto al tuo target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Importa entrambi i moduli dove necessario:

```swift
import FastCommentsUI
import FastCommentsSwift
```