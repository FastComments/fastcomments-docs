### Swift Package Manager

Dodajte naslednje v svojo datoteko `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.2")
]
```

Ali v Xcode:
1. File > Add Packages...
2. Vnesite URL repozitorija: `https://github.com/fastcomments/fastcomments-swift.git`
3. Izberite želeno različico

### Zahteve

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+