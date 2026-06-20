### Swift Package Manager

Añade lo siguiente a tu archivo `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "2.0.0")
]
```

O en Xcode:
1. Archivo > Añadir paquetes...
2. Introduce la URL del repositorio: `https://github.com/fastcomments/fastcomments-swift.git`
3. Selecciona la versión que deseas usar

### Requisitos

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+