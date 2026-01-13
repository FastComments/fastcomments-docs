### Swift Package Manager

Dodajte naslednje v datoteko `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

Ali v Xcode:
1. Datoteka > Dodaj pakete...
2. Vnesite URL repozitorija: `https://github.com/fastcomments/fastcomments-swift.git`
3. Izberite različico, ki jo želite uporabiti

### Zahteve

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+
---