### Swift Package Manager

Voeg het volgende toe aan uw `Package.swift`-bestand:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

Of in Xcode:
1. Bestand > Voeg pakketten toe...
2. Voer de repository-URL in: `https://github.com/fastcomments/fastcomments-swift.git`
3. Selecteer de versie die u wilt gebruiken

### Vereisten

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+