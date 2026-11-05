### Swift Package Manager

Aggiungi quanto segue al tuo file `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.2")
]
```

Oppure in Xcode:
1. File > Add Packages...
2. Inserisci l'URL del repository: `https://github.com/fastcomments/fastcomments-swift.git`
3. Seleziona la versione che vuoi utilizzare

### Requisiti

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+