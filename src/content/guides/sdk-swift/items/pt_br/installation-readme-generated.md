### Swift Package Manager

Adicione o seguinte ao seu arquivo `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.0")
]
```

Ou no Xcode:
1. File > Add Packages...
2. Insira a URL do repositório: `https://github.com/fastcomments/fastcomments-swift.git`
3. Selecione a versão que deseja usar

### Requirements

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+