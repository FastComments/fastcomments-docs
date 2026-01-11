### Gerenciador de Pacotes Swift

Adicione o seguinte ao seu arquivo `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

Ou no Xcode:
1. File > Add Packages...
2. Insira a URL do repositório: `https://github.com/fastcomments/fastcomments-swift.git`
3. Selecione a versão que deseja usar

### Requisitos

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+