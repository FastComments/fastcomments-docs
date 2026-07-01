Adicione o FastCommentsUI ao seu projeto usando o Swift Package Manager.

No Xcode: **File > Add Package Dependencies**, em seguida, insira a URL do repositório.

Ou adicione ao seu `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

Então adicione o produto ao seu target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Importe ambos os módulos onde for necessário:

```swift
import FastCommentsUI
import FastCommentsSwift
```