### 广播 ID

你会看到在某些 API 调用中需要传入 `broadcastId`。当你接收到事件时，会返回该 ID，这样如果你打算在客户端乐观地应用更改（你很可能会这么做，因为它能提供最佳体验），就可以知道要忽略该事件。请在此处传入一个 UUID。该 ID 应该足够唯一，以避免在一次会话中出现两次。

```swift
let broadcastId = UUID().uuidString
```