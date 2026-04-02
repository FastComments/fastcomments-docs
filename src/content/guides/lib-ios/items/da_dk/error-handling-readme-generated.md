SDK-metoder kaster `FastCommentsError`, som overholder `LocalizedError`:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` egenskaber:

- `code` -- fejlkode fra API'en
- `reason` -- engelsk fejlbeskrivelse
- `translatedError` -- lokaliseret fejlmeddelelse leveret af serveren

Blokeringsfejl vises også automatisk via `sdk.blockingErrorMessage`, som de indbyggede visninger viser for brugeren.

---
---