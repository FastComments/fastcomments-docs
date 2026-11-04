SDK методите хвърлят `FastCommentsError`, който съответства на `LocalizedError`:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

Свойства на `FastCommentsError`:

- `code` -- код на грешката от API
- `reason` -- описание на грешката на английски
- `translatedError` -- локализирано съобщение за грешка, предоставено от сървъра

Грешките при блокиране също се показват автоматично чрез `sdk.blockingErrorMessage`, което вградените изгледи показват на потребителя.

---