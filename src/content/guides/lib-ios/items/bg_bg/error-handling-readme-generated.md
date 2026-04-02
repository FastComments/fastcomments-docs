Методите на SDK хвърлят `FastCommentsError`, който съответства на `LocalizedError`:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` свойства:

- `code` -- код на грешка от API-то
- `reason` -- описание на грешката на английски
- `translatedError` -- локализирано съобщение за грешка, предоставено от сървъра

Blocking errors are also surfaced automatically via `sdk.blockingErrorMessage`, which the built-in views display to the user.

---
---