FastComments तीन प्रमाणीकरण मोड का समर्थन करता है:

1. **Anonymous** -- कोई SSO टोकन नहीं; उपयोगकर्ताओं को सेशन-आधारित पहचान मिलती है
2. **Simple SSO** -- डेमो और टेस्टिंग के लिए क्लाइंट-साइड टोकन (सुरक्षित नहीं)
3. **Secure SSO** -- प्रोडक्शन के लिए सर्वर-साइन्ड टोकन

### Simple SSO

डेमो और लोकल टेस्टिंग के लिए उपयोगी। Simple SSO के साथ कोई भी किसी भी उपयोगकर्ता की नकल कर सकता है, इसलिए इसे प्रोडक्शन में उपयोग न करें।

```swift
import FastCommentsSwift

let userData = SimpleSSOUserData(
    username: "Jane Doe",
    email: "jane@example.com",
    avatar: "https://example.com/avatar.jpg"
)
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
let token = try? sso.prepareToSend()

let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page-1",
    sso: token
)
let sdk = FastCommentsSDK(config: config)
```

`SimpleSSOUserData` वैकल्पिक फ़ील्ड का भी समर्थन करता है:

- `id` -- उपयोगकर्ता ID (सेट न होने पर डिफ़ॉल्ट रूप से ईमेल)
- `displayName` -- अलग डिस्प्ले नाम
- `displayLabel` -- नाम के आगे दिखाया जाने वाला कस्टम लेबल (जैसे "VIP")
- `websiteUrl` -- उपयोगकर्ता के नाम पर लिंक
- `locale` -- लोकेल कोड
- `isProfileActivityPrivate` -- प्रोफ़ाइल गतिविधि छिपाएँ (डिफ़ॉल्ट रूप से true)

### Secure SSO

प्रोडक्शन में, आपका बैकएंड आपके API सीक्रेट का उपयोग करके एक साइन्ड SSO टोकन जनरेट करता है। iOS ऐप आपके सर्वर से यह टोकन प्राप्त करता है और इसे कॉन्फ़िग में पास करता है।

**आपके बैकएंड पर** (FastComments Swift SDK या किसी भी भाषा का उपयोग करते हुए):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// यह टोकन अपने API के माध्यम से अपने iOS ऐप को लौटाएँ
```

**अपने iOS ऐप में:**

```swift
struct MyView: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "my-page-1"
        )
    )
    @State private var isLoadingToken = true

    var body: some View {
        Group {
            if isLoadingToken {
                ProgressView("Loading...")
            } else {
                FastCommentsView(sdk: sdk)
            }
        }
        .task {
            // अपने बैकएंड से टोकन प्राप्त करें
            let token = try? await fetchSSOTokenFromYourBackend()
            // टोकन के साथ एक नया कॉन्फ़िग बनाएँ, या लोड से पहले सेट करें
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` अतिरिक्त फ़ील्ड का समर्थन करता है:

- `optedInNotifications` -- ईमेल नोटिफिकेशन ऑप्ट-इन
- `displayLabel` -- कस्टम लेबल
- `displayName` -- डिस्प्ले नाम
- `websiteUrl` -- वेबसाइट URL
- `groupIds` -- समूह सदस्यताएँ
- `isAdmin` -- एडमिन विशेषाधिकार
- `isModerator` -- मॉडरेटर विशेषाधिकार
- `isProfileActivityPrivate` -- प्रोफ़ाइल गोपनीयता

---
---