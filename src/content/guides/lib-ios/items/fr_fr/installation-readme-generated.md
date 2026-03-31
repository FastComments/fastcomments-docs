Ajoutez FastCommentsUI à votre projet en utilisant Swift Package Manager.

Dans Xcode : **File > Add Package Dependencies**, puis saisissez l'URL du dépôt.

Ou ajoutez-le à votre `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Then add the product to your target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Importez les deux modules là où nécessaire :

```swift
import FastCommentsUI
import FastCommentsSwift
```