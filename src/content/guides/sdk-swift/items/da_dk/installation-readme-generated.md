### Swift Package Manager

Tilføj følgende til din `Package.swift`-fil:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "2.0.0")
]
```

Eller i Xcode:
1. Vælg File > Add Packages...
2. Indtast repository-URL'en: `https://github.com/fastcomments/fastcomments-swift.git`
3. Vælg den version, du vil bruge

### Krav

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+