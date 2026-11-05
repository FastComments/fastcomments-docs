### Swift Package Manager

`Package.swift` 파일에 다음을 추가하세요:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.2")
]
```

또는 Xcode에서:
1. File > Add Packages... 선택
2. 저장소 URL 입력: `https://github.com/fastcomments/fastcomments-swift.git`
3. 사용하려는 버전 선택

### 요구 사항

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+