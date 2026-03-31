---
SDK-Methoden werfen `FastCommentsError`, der `LocalizedError` entspricht:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` Eigenschaften:

- `code` -- Fehlercode von der API
- `reason` -- Englische Fehlerbeschreibung
- `translatedError` -- vom Server bereitgestellte lokalisierte Fehlermeldung

Blockierende Fehler werden auch automatisch über `sdk.blockingErrorMessage` bereitgestellt, die von den integrierten Ansichten dem Benutzer angezeigt werden.

---
---