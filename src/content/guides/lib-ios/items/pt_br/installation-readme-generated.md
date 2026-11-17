Adicione o FastCommentsUI ao seu projeto usando o Swift Package Manager.

No Xcode: **File > Add Package Dependencies**, depois insira a URL do repositório.

Ou adicione ao seu `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

Em seguida, adicione o produto ao seu target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Importe ambos os módulos onde necessário:

```swift
import FastCommentsUI
import FastCommentsSwift
```