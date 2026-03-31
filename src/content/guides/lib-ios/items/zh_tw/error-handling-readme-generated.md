SDK 方法會拋出 `FastCommentsError`，它符合 `LocalizedError`：

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` 屬性：

- `code` -- 來自 API 的錯誤代碼
- `reason` -- 英文錯誤描述
- `translatedError` -- 伺服器提供的本地化錯誤訊息

封鎖錯誤也會自動透過 `sdk.blockingErrorMessage` 顯示出來，內建視圖會將其展示給使用者。

---
---