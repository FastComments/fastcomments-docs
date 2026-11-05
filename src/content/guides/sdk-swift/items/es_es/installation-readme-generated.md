### Swift Package Manager

Añade lo siguiente a tu archivo `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.2")
]
```

O en Xcode:
1. File > Add Packages...
2. Introduce la URL del repositorio: `https://github.com/fastcomments/fastcomments-swift.git`
3. Selecciona la versión que quieras usar

### Requisitos

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+