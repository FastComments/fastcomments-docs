### Swift パッケージマネージャ

以下を `Package.swift` ファイルに追加してください:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.0.0")
]
```

または Xcode で:
1. File > パッケージを追加...
2. リポジトリ URL を入力してください: `https://github.com/fastcomments/fastcomments-swift.git`
3. 使用したいバージョンを選択してください

### 要件

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+