EU डेटा सेंटर का उपयोग करने के लिए, अपने कॉन्फ़िग में `region` फ़ील्ड सेट करें:

```swift
let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page",
    region: "eu"
)
```

यह सभी API अनुरोधों और WebSocket कनेक्शनों को `eu.fastcomments.com` पर रूट करता है।