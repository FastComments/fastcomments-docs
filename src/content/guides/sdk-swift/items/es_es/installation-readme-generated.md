### Swift Package Manager

Agrega lo siguiente a tu archivo `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "1.2.1")
]
```

O en Xcode:
1. Archivo > Agregar paquetes...
2. Introduce la URL del repositorio: `https://github.com/fastcomments/fastcomments-swift.git`
3. Selecciona la versión que quieres usar

### Requisitos

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+