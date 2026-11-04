SDK metode vržejo `FastCommentsError`, ki sledi `LocalizedError`:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

Lastnosti `FastCommentsError`:

- `code` -- koda napake iz API-ja
- `reason` -- opis napake v angleščini
- `translatedError` -- lokalizirano sporočilo o napaki, ki ga zagotovi strežnik

Napake pri blokiranju se prikažejo tudi samodejno prek `sdk.blockingErrorMessage`, ki jih vgrajeni pogledi prikažejo uporabniku.

---