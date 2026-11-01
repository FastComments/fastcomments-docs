### Upravitelj Swift paketa

Dodajte sljedeće u svoju datoteku `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.1")
]
```

Ili u Xcodeu:
1. File > Add Packages...
2. Unesite URL repozitorija: `https://github.com/fastcomments/fastcomments-swift.git`
3. Odaberite verziju koju želite koristiti

### Zahtjevi

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+