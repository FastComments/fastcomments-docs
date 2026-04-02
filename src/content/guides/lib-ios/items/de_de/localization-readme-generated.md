Geben Sie einen Locale-Code in der Konfiguration an, um serverseitig bereitgestellte Zeichenketten zu lokalisieren:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    locale: "fr_fr"
)
```

Clientseitige UI-Zeichenketten verwenden die auf iOS-Bundles basierende Lokalisierung.