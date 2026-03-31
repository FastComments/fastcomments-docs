SDK metode bacaju `FastCommentsError`, koji implementira `LocalizedError`:

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

- `code` -- kod greške iz API-ja
- `reason` -- opis greške na engleskom jeziku
- `translatedError` -- lokalizovana poruka o grešci koju pruža server

Blocking errors are also surfaced automatically via `sdk.blockingErrorMessage`, which the built-in views display to the user.

---
---