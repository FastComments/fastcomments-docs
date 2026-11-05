### Swift Package Manager

Προσθέστε τα εξής στο αρχείο `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.2")
]
```

Ή στο Xcode:
1. File > Add Packages...
2. Εισάγετε το URL του repository: `https://github.com/fastcomments/fastcomments-swift.git`
3. Επιλέξτε την έκδοση που θέλετε να χρησιμοποιήσετε

### Απαιτήσεις

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+