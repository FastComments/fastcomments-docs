SDK のメソッドは `FastCommentsError` をスローします。これは `LocalizedError` に準拠しています:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` のプロパティ:

- `code` -- API からのエラーコード
- `reason` -- 英語のエラー説明
- `translatedError` -- サーバーから提供されるローカライズされたエラーメッセージ

ブロッキングエラーは `sdk.blockingErrorMessage` を通じて自動的に通知され、組み込みのビューがユーザーに表示します。

---
---