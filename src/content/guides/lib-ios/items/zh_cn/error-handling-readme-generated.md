SDK 方法会抛出 `FastCommentsError`，它符合 `LocalizedError`：

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` 的属性：

- `code` -- 来自 API 的错误代码
- `reason` -- 英文错误描述
- `translatedError` -- 服务器提供的本地化错误消息

阻塞错误也会通过 `sdk.blockingErrorMessage` 自动呈现，内置视图会向用户显示该消息。

---
---