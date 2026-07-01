### Swift Package Manager

Dodaj poniższy kod do swojego pliku `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.0.0")
]
```

Lub w Xcode:
1. Plik > Dodaj pakiety...
2. Wprowadź URL repozytorium: `https://github.com/fastcomments/fastcomments-swift.git`
3. Wybierz wersję, której chcesz używać

### Requirements

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+