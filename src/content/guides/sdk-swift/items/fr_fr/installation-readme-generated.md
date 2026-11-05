### Swift Package Manager

Ajoutez ce qui suit à votre fichier `Package.swift` :

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.2")
]
```

Ou dans Xcode :
1. File > Add Packages...
2. Saisissez l'URL du dépôt : `https://github.com/fastcomments/fastcomments-swift.git`
3. Sélectionnez la version que vous souhaitez utiliser

### Prérequis

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+