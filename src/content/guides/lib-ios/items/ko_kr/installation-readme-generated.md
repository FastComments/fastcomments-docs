Swift Package Manager를 사용하여 프로젝트에 FastCommentsUI를 추가합니다.

Xcode에서: **File > Add Package Dependencies**를 선택한 다음, 저장소 URL을 입력합니다.

또는 `Package.swift`에 추가합니다:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

그런 다음 타겟에 product를 추가합니다:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

필요한 곳에서 두 모듈을 모두 import합니다:

```swift
import FastCommentsUI
import FastCommentsSwift
```