---
SDK 메서드는 `FastCommentsError`를 던지며, 이는 `LocalizedError`를 준수합니다:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` 속성:

- `code` -- API의 오류 코드
- `reason` -- 영어 오류 설명
- `translatedError` -- 서버에서 제공한 현지화된 오류 메시지

차단 오류는 또한 `sdk.blockingErrorMessage`를 통해 자동으로 노출되며, 내장 뷰가 이를 사용자에게 표시합니다.

---