### Swift Package Manager

Dodajte sljedeće u vašu `Package.swift` datoteku:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

Ili u Xcode-u:
1. File > Add Packages...
2. Unesite URL repozitorija: `https://github.com/fastcomments/fastcomments-swift.git`
3. Odaberite verziju koju želite koristiti

### Zahtjevi

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+