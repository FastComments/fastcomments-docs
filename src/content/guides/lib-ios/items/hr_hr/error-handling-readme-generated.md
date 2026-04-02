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

- `code` -- kod pogreške iz API-ja
- `reason` -- engleski opis pogreške
- `translatedError` -- lokalizirana poruka o pogrešci koju pruža server

Blokirajuće pogreške također se automatski prikazuju putem `sdk.blockingErrorMessage`, koje ugrađeni prikazi prikazuju korisniku.

---
---