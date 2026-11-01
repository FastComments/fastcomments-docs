### Swift パッケージマネージャ

`Package.swift` ファイルに以下を追加してください:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.1")
]
```

Xcode で追加する場合:
1. File > Add Packages...
2. リポジトリ URL を入力してください: `https://github.com/fastcomments/fastcomments-swift.git`
3. 使用したいバージョンを選択してください

### 必要条件

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+