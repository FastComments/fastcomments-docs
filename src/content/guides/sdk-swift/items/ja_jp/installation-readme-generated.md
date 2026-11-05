### Swift Package Manager

`Package.swift` ファイルに以下を追加します:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.2")
]
```

または、Xcode では:
1. File > Add Packages... を選択
2. リポジトリの URL を入力: `https://github.com/fastcomments/fastcomments-swift.git`
3. 使用したいバージョンを選択

### 要件

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+