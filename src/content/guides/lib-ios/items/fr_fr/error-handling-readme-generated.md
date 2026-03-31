Les méthodes du SDK lèvent `FastCommentsError`, qui est conforme à `LocalizedError`:

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

- `code` -- code d'erreur provenant de l'API
- `reason` -- description de l'erreur en anglais
- `translatedError` -- message d'erreur localisé fourni par le serveur

Blocking errors are also surfaced automatically via `sdk.blockingErrorMessage`, which the built-in views display to the user.

---
---