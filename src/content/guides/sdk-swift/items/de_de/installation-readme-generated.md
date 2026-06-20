### Swift Package Manager

Fügen Sie Folgendes zu Ihrer `Package.swift`-Datei hinzu:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "2.0.0")
]
```

Oder in Xcode:
1. File > Add Packages...
2. Geben Sie die Repository-URL ein: `https://github.com/fastcomments/fastcomments-swift.git`
3. Wählen Sie die gewünschte Version aus

### Anforderungen

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+