कॉन्फ़िग में एक locale कोड पास करें ताकि सर्वर-प्रदत्त स्ट्रिंग्स का स्थानीयकरण हो सके:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    locale: "fr_fr"
)
```

क्लाइंट-साइड UI स्ट्रिंग्स iOS बंडल-आधारित स्थानीयकरण का उपयोग करती हैं।