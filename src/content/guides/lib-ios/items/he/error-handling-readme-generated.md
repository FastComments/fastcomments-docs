שיטות SDK זורקות `FastCommentsError`, שמעומדת ל-`LocalizedError`:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` מאפיינים:

- `code` -- קוד שגיאה מה-API
- `reason` -- תיאור השגיאה באנגלית
- `translatedError` -- הודעת שגיאה מתורגמת המסופקת על ידי השרת

שגיאות החוסמות גם מוצגות אוטומטית באמצעות `sdk.blockingErrorMessage`, שהתצוגות המובנות מציגות למשתמש.

---
---