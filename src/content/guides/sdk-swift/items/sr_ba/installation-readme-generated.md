### Swift Package Manager

Dodajte sljedeće u vaš `Package.swift` fajl:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.0.0")
]
```

Ili u Xcode-u:
1. File > Dodaj pakete...
2. Unesite URL repozitorija: `https://github.com/fastcomments/fastcomments-swift.git`
3. Odaberite verziju koju želite koristiti

### Zahtjevi

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+