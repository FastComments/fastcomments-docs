Ajoutez FastCommentsUI à votre projet en utilisant le Gestionnaire de paquets Swift.

Dans Xcode : **Fichier > Ajouter des dépendances de paquet**, puis entrez l'URL du dépôt.

Ou ajoutez-le à votre `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Ensuite, ajoutez le produit à votre cible:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Importez les deux modules là où nécessaire:

```swift
import FastCommentsUI
import FastCommentsSwift
```