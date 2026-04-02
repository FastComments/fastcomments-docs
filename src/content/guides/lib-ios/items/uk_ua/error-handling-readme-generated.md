Методи SDK викидають `FastCommentsError`, який відповідає протоколу `LocalizedError`:

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

- `code` -- код помилки з API
- `reason` -- опис помилки англійською мовою
- `translatedError` -- локалізоване повідомлення про помилку, надане сервером

Блокувальні помилки також автоматично надаються через `sdk.blockingErrorMessage`, і вбудовані перегляди відображають їх користувачеві.

---
---