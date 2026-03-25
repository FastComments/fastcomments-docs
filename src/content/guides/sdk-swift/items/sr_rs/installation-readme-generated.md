### Swift Package Manager

Dodajte sledeće u vaš `Package.swift` fajl:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "1.2.1")
]
```

Ili u Xcode-u:
1. Datoteka > Dodaj pakete...
2. Unesite URL repozitorijuma: `https://github.com/fastcomments/fastcomments-swift.git`
3. Izaberite verziju koju želite da koristite

### Zahtevi

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+
---