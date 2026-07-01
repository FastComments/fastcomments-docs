### Swift-Paket-Manager

Fügen Sie das Folgende zu Ihrer `Package.swift`‑Datei hinzu:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.0.0")
]
```

Oder in Xcode:
1. Datei > Pakete hinzufügen...
2. Geben Sie die Repository‑URL ein: `https://github.com/fastcomments/fastcomments-swift.git`
3. Wählen Sie die Version, die Sie verwenden möchten

### Voraussetzungen

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+