Añade FastCommentsUI a tu proyecto usando Swift Package Manager.

En Xcode: **Archivo > Añadir dependencias de paquetes**, luego introduce la URL del repositorio.

O añádelo a tu `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Luego añade el producto a tu target:

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