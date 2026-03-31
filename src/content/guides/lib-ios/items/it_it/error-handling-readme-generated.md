I metodi dell'SDK lanciano `FastCommentsError`, che aderisce al protocollo `LocalizedError`:

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

- `code` -- codice di errore restituito dall'API
- `reason` -- descrizione dell'errore in inglese
- `translatedError` -- messaggio di errore localizzato fornito dal server

Anche gli errori bloccanti vengono esposti automaticamente tramite `sdk.blockingErrorMessage`, che le viste integrate mostrano all'utente.

---
---