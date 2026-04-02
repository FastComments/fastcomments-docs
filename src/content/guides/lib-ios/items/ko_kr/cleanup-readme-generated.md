SDK 인스턴스 사용을 마쳤을 때(예: 뷰가 닫히는 경우), WebSocket 연결을 닫고 백그라운드 작업을 취소하려면 `cleanup()`을 호출하세요:

```swift
sdk.cleanup()
```

SwiftUI의 `@StateObject`로 관리되는 뷰의 경우, 일반적으로 `.onDisappear`에서 호출하거나 뷰가 해제될 때 호출합니다.