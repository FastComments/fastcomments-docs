### Swift Package Manager

Dodaj następujący kod do pliku `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.1")
]
```

Lub w Xcode:
1. File > Add Packages...
2. Wprowadź adres URL repozytorium: `https://github.com/fastcomments/fastcomments-swift.git`
3. Wybierz wersję, której chcesz użyć

### Wymagania

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+