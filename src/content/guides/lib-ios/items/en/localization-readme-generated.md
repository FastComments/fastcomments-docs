Pass a locale code in the config to localize server-provided strings:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    locale: "fr_fr"
)
```

Client-side UI strings use iOS bundle-based localization.

---