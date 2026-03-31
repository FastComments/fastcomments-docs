Методе SDK-а бацају `FastCommentsError`, који имплементира `LocalizedError`:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` својства:

- `code` -- код грешке из API
- `reason` -- енглески опис грешке
- `translatedError` -- локализована порука о грешци коју обезбеђује сервер

Блокирајуће грешке се такође аутоматски приказују преко `sdk.blockingErrorMessage`, које уграђени прикази показују кориснику.

---
---