---
### Swift パッケージマネージャ

次を `Package.swift` ファイルに追加してください:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "2.0.0")
]
```

または Xcode で:
1. ファイル > パッケージを追加...
2. リポジトリの URL を入力: `https://github.com/fastcomments/fastcomments-swift.git`
3. 使用したいバージョンを選択

### 要件

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+
---