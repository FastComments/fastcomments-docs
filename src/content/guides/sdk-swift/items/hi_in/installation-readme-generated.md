### Swift Package Manager

अपनी `Package.swift` फ़ाइल में निम्नलिखित जोड़ें:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.2")
]
```

या Xcode में:
1. File > Add Packages...
2. रिपॉज़िटरी URL दर्ज करें: `https://github.com/fastcomments/fastcomments-swift.git`
3. वह संस्करण चुनें जिसे आप उपयोग करना चाहते हैं

### आवश्यकताएँ

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+