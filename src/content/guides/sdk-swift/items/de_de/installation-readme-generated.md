---
### Swift-Paket-Manager

Fügen Sie Folgendes zu Ihrer Datei `Package.swift` hinzu:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "1.2.1")
]
```

Oder in Xcode:
1. Datei > Pakete hinzufügen...
2. Geben Sie die Repository-URL ein: `https://github.com/fastcomments/fastcomments-swift.git`
3. Wählen Sie die gewünschte Version aus

### Anforderungen

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+
---