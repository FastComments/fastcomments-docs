---
Adicione FastCommentsUI ao seu projeto usando o Swift Package Manager.

No Xcode: **Arquivo > Adicionar Dependências de Pacote**, em seguida insira a URL do repositório.

Ou adicione-o ao seu `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
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
---