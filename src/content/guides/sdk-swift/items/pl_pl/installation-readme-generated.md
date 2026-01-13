---
### Menedżer pakietów Swift

Dodaj poniższe do pliku `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

Lub w Xcode:
1. Plik > Dodaj pakiety...
2. Wpisz adres URL repozytorium: `https://github.com/fastcomments/fastcomments-swift.git`
3. Wybierz wersję, której chcesz użyć

### Wymagania

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+
---