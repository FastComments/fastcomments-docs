### Swift Package Manager

Add the following to your `Package.swift` file:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "1.2.0")
]
```

Or in Xcode:
1. File > Add Packages...
2. Enter the repository URL: `https://github.com/fastcomments/fastcomments-swift.git`
3. Select the version you want to use

### Requirements

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+