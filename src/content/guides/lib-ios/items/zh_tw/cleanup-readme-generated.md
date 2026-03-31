---
當您完成使用 SDK 實例時（例如視圖即將被關閉），呼叫 `cleanup()` 以關閉 WebSocket 連線並取消背景任務：

```swift
sdk.cleanup()
```

對於由 SwiftUI 的 `@StateObject` 管理的視圖，通常會在 `.onDisappear` 或視圖被釋放（deallocated）時呼叫。

---
---