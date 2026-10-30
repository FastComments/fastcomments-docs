### Swift Package Manager

Dodaj poniższe do swojego pliku `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.0")
]
```

Lub w Xcode:
1. Plik > Dodaj pakiety...
2. Wprowadź adres URL repozytorium: `https://github.com/fastcomments/fastcomments-swift.git`
3. Wybierz wersję, której chcesz użyć

### Requirements

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+