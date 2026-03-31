Καταχωρήστε έναν κωδικό locale στο config για να τοπικοποιήσετε τις συμβολοσειρές που παρέχει ο διακομιστής:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    locale: "fr_fr"
)
```

Οι συμβολοσειρές διεπαφής χρήστη στην πλευρά του πελάτη χρησιμοποιούν τοπικοποίηση βασισμένη σε iOS bundles.