### Swift パッケージマネージャー

次の内容を `Package.swift` ファイルに追加してください：

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

または Xcode で：
1. ファイル > パッケージを追加...
2. リポジトリの URL を入力してください: `https://github.com/fastcomments/fastcomments-swift.git`
3. 使用するバージョンを選択します

### 要件

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+