### Διαχειριστής Πακέτων Swift

Προσθέστε τα παρακάτω στο αρχείο `Package.swift` σας:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.0.0")
]
```

Ή στο Xcode:
1. Αρχείο > Προσθήκη Πακέτων...
2. Εισάγετε το URL του αποθετηρίου: `https://github.com/fastcomments/fastcomments-swift.git`
3. Επιλέξτε την έκδοση που θέλετε να χρησιμοποιήσετε

### Απαιτήσεις

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+