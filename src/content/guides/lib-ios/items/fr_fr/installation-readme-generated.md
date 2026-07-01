Ajoutez FastCommentsUI à votre projet en utilisant Swift Package Manager.

Dans Xcode : **File > Add Package Dependencies**, puis saisissez l’URL du dépôt.

Ou ajoutez‑le à votre `Package.swift` :

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

Puis ajoutez le produit à votre cible :

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Importez les deux modules là où c’est nécessaire :

```swift
import FastCommentsUI
import FastCommentsSwift
```