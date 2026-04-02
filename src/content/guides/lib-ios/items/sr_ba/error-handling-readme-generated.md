SDK методе бацају `FastCommentsError`, који имплементира `LocalizedError`:

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

- `code` -- код грешке из API-ја
- `reason` -- опис грешке на енглеском
- `translatedError` -- локализована порука о грешци обезбеђена од стране сервера

Blocking errors are also surfaced automatically via `sdk.blockingErrorMessage`, which the built-in views display to the user.

---
---