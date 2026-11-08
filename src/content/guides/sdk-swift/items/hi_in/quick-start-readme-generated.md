### पब्लिक API का उपयोग करना

```swift
import FastCommentsSwift

// किसी पेज के लिए कमेंट्स प्राप्त करें
do {
    let response = try await PublicAPI.getCommentsPublic(
        tenantId: "your-tenant-id",
        urlId: "page-url-id"
    )

    print("Found \(response.comments?.count ?? 0) comments")
    for comment in response.comments ?? [] {
        print("Comment: \(comment.comment ?? "")")
    }
} catch {
    print("Error fetching comments: \(error)")
}
```

### ऑथेंटिकेटेड API का उपयोग करना

```swift
import FastCommentsSwift

// शेयर्ड कॉन्फ़िगरेशन पर अपनी API key कॉन्फ़िगर करें (x-api-key हेडर के रूप में भेजी जाती है)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// ऑथेंटिकेटेड API का उपयोग करके कमेंट्स प्राप्त करें
do {
    let response = try await DefaultAPI.getComments(
        tenantId: "your-tenant-id",
        options: .init(urlId: "page-url-id")
    )

    print("Total comments: \(response.count ?? 0)")
    for comment in response.comments ?? [] {
        print("Comment ID: \(comment.id ?? ""), Text: \(comment.comment ?? "")")
    }
} catch {
    print("Error: \(error)")
}
```

### मॉडरेशन API का उपयोग करना

```swift
import FastCommentsSwift

// मॉडरेशन मेथड्स को कार्यरत मॉडरेटर के लिए `sso` टोकन से अधिकृत किया जाता है
// (इसे FastCommentsSSO से जनरेट करें, ऊपर SSO सेक्शन देखें)।
do {
    let response = try await ModerationAPI.getApiComments(
        options: .init(
            page: 0,
            count: 30,
            sso: ssoToken
        )
    )

    print("Found \(response.comments.count) comments to moderate")
    for comment in response.comments {
        print("Comment ID: \(comment.id), Text: \(comment.commentHTML)")
    }
} catch {
    print("Error: \(error)")
}
```

### ऑथेंटिकेशन के लिए SSO का उपयोग करना

#### सिक्योर SSO (प्रोडक्शन के लिए अनुशंसित)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// सिक्योर SSO यूज़र डेटा बनाएं (केवल सर्वर-साइड!)
let userData = SecureSSOUserData(
    id: "user-123",              // यूज़र ID
    email: "user@example.com",   // ईमेल
    username: "johndoe",         // यूज़रनेम
    avatar: "https://example.com/avatar.jpg" // अवतार URL
)

// SSO टोकन जनरेट करें
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // ऑथेंटिकेशन के लिए यह टोकन अपने फ्रंटएंड को पास करें
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### सिंपल SSO (डेवलपमेंट/टेस्टिंग के लिए)

```swift
import FastCommentsSwift

// सिंपल SSO यूज़र डेटा बनाएं (API key की आवश्यकता नहीं)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// सिंपल SSO टोकन जनरेट करें
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```