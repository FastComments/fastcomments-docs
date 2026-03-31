SDK metode bacaju `FastCommentsError`, koji se pridržava protokola `LocalizedError`:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` svojstva:

- `code` -- kod greške iz API-ja
- `reason` -- opis greške na engleskom jeziku
- `translatedError` -- lokalizovana poruka o grešci koju pruža server

Blokirajuće greške se takođe automatski prikazuju preko `sdk.blockingErrorMessage`, a ugrađeni prikazi ih prikazuju korisniku.

---
---