Agrega FastCommentsUI a tu proyecto usando Swift Package Manager.

En Xcode: **File > Add Package Dependencies**, luego introduce la URL del repositorio.

O agrégalo a tu `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

Luego agrega el producto a tu target:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Importa ambos módulos donde sea necesario:

```swift
import FastCommentsUI
import FastCommentsSwift
```