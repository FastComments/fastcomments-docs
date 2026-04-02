Metode SDK vržejo `FastCommentsError`, ki je skladna z `LocalizedError`:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` lastnosti:

- `code` -- koda napake iz API-ja
- `reason` -- angleški opis napake
- `translatedError` -- strežniško posredovano lokalizirano sporočilo o napaki

Blocking errors are also surfaced automatically via `sdk.blockingErrorMessage`, which the built-in views display to the user.

---
---