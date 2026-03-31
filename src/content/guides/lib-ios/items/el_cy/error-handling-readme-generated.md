Οι μέθοδοι του SDK ρίχνουν `FastCommentsError`, το οποίο συμμορφώνεται με το `LocalizedError`:

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

- `code` -- κωδικός σφάλματος από το API
- `reason` -- περιγραφή σφάλματος στα Αγγλικά
- `translatedError` -- τοπικοποιημένο μήνυμα σφάλματος που παρέχεται από τον διακομιστή

Τα σφάλματα αποκλεισμού προβάλλονται επίσης αυτόματα μέσω του `sdk.blockingErrorMessage`, το οποίο οι ενσωματωμένες προβολές εμφανίζουν στον χρήστη.

---
---