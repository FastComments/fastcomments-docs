Методы SDK выбрасывают `FastCommentsError`, который соответствует `LocalizedError`:

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

- `code` -- код ошибки из API
- `reason` -- описание ошибки на английском языке
- `translatedError` -- локализованное сообщение об ошибке, предоставленное сервером

Ошибки блокировки также автоматически отображаются через `sdk.blockingErrorMessage`, которое встроенные представления показывают пользователю.