当您完成对 SDK 实例的使用（例如视图即将被关闭）时，调用 `cleanup()` 来关闭 WebSocket 连接并取消后台任务：

```swift
sdk.cleanup()
```

对于由 SwiftUI 的 `@StateObject` 管理的视图，通常在 `.onDisappear` 中调用，或在视图被释放时调用。

---
---