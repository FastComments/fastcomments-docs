### Swift Package Manager

Dodajte naslednje v vašo datoteko `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.0")
]
```

Ali v Xcode-u:
1. File > Add Packages...
2. Vnesite URL repozitorija: `https://github.com/fastcomments/fastcomments-swift.git`
3. Izberite različico, ki jo želite uporabiti

### Zahteve

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+