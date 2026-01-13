### Swift 패키지 관리자

다음 내용을 `Package.swift` 파일에 추가하세요:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

또는 Xcode에서:
1. 파일 > 패키지 추가...
2. 저장소 URL을 입력하세요: `https://github.com/fastcomments/fastcomments-swift.git`
3. 사용하려는 버전을 선택하세요

### 요구 사항

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+