---
SDK-methoden gooien `FastCommentsError`, die voldoet aan `LocalizedError`:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

Eigenschappen van `FastCommentsError`:

- `code` -- foutcode van de API
- `reason` -- Engelse foutbeschrijving
- `translatedError` -- door de server geleverde gelokaliseerde foutmelding

Blokkerende fouten worden ook automatisch zichtbaar gemaakt via `sdk.blockingErrorMessage`, die door de ingebouwde weergaven aan de gebruiker wordt weergegeven.

---
---