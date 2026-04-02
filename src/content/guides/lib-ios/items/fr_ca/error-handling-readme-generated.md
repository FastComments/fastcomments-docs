Les méthodes du SDK lancent `FastCommentsError`, qui se conforme à `LocalizedError` :

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

- `code` -- code d'erreur renvoyé par l'API
- `reason` -- description de l'erreur en anglais
- `translatedError` -- message d'erreur localisé fourni par le serveur

Les erreurs bloquantes sont également remontées automatiquement via `sdk.blockingErrorMessage`, que les vues intégrées affichent à l'utilisateur.

---
---