SDK methods throw `FastCommentsError`, which conforms to `LocalizedError`:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` properties:

- `code` -- код ошибки от API
- `reason` -- описание ошибки на английском языке
- `translatedError` -- локализованное сообщение об ошибке, предоставленное сервером

Blocking errors are also surfaced automatically via `sdk.blockingErrorMessage`, which the built-in views display to the user.

---
---