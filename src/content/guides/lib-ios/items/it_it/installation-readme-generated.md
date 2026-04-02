---
Aggiungi FastCommentsUI al tuo progetto utilizzando Swift Package Manager.

In Xcode: **File > Aggiungi dipendenze del pacchetto**, quindi inserisci l'URL del repository.

Oppure aggiungilo al tuo `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
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
---