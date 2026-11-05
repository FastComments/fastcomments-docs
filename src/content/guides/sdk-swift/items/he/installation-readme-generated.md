### Swift Package Manager

הוסיפו את הבא לקובץ `Package.swift` שלכם:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.2")
]
```

או ב-Xcode:
1. File > Add Packages...
2. הזינו את כתובת ה-URL של המאגר: `https://github.com/fastcomments/fastcomments-swift.git`
3. בחרו את הגרסה שבה ברצונכם להשתמש

### דרישות

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+