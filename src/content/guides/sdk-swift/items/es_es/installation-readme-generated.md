---
### Swift Package Manager

Agregue lo siguiente a su archivo `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

O en Xcode:
1. Archivo > Añadir paquetes...
2. Ingrese la URL del repositorio: `https://github.com/fastcomments/fastcomments-swift.git`
3. Seleccione la versión que desea usar

### Requisitos

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+
---