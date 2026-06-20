---
### Swift Package Manager

Dodajte sljedeće u datoteku `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "2.0.0")
]
```

Ili u Xcodeu:
1. Datoteka > Dodaj pakete...
2. Unesite URL repozitorija: `https://github.com/fastcomments/fastcomments-swift.git`
3. Odaberite verziju koju želite koristiti

### Zahtjevi

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+
---