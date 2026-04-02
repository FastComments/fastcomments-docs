---
Metody SDK zgłaszają `FastCommentsError`, który implementuje `LocalizedError`:

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

- `code` -- kod błędu z API
- `reason` -- opis błędu w języku angielskim
- `translatedError` -- lokalizowany komunikat o błędzie dostarczony przez serwer

Błędy blokujące są również automatycznie udostępniane przez `sdk.blockingErrorMessage`, a wbudowane widoki wyświetlają je użytkownikowi.

---
---