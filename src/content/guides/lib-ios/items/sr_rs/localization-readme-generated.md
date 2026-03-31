---
Prosledite kod lokaliteta (locale) u konfiguraciji da biste lokalizovali stringove koje obezbeđuje server:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    locale: "fr_fr"
)
```

Tekstovi korisničkog interfejsa na klijentskoj strani koriste lokalizaciju zasnovanu na iOS bundle-u.
---
---