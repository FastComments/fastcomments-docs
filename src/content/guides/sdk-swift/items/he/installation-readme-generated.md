---
### מנהל החבילות של Swift

הוסף את הדברים הבאים לקובץ `Package.swift` שלך:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "1.2.1")
]
```

או ב‑Xcode:
1. קובץ > הוספת חבילות...
2. הזן את כתובת המאגר: `https://github.com/fastcomments/fastcomments-swift.git`
3. בחר את הגרסה שבה ברצונך להשתמש

### דרישות

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+
---