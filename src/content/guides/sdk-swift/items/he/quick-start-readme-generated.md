### שימוש ב‑API ציבורי

```swift
import FastCommentsSwift

// הפקת תגובות עבור עמוד
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

### שימוש ב‑API מאומת

```swift
import FastCommentsSwift

// הגדר את מפתח ה‑API שלך בתצורה המשותפת (נשלח ככותרת x-api-key header)
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "your-api-key"

// הפקת תגובות באמצעות API מאומת
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

### שימוש ב‑API מודרציה

```swift
import FastCommentsSwift

// שיטות מודרציה מאושרות בעזרת אסימון `sso` עבור המנחה הפועל
// (צור אותו עם FastCommentsSSO, ראה את סעיף ה‑SSO למעלה).
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

### שימוש ב‑SSO לאימות

#### SSO מאובטח (מומלץ לייצור)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// צור נתוני משתמש SSO מאובטח (רק בשרת!)
let userData = SecureSSOUserData(
    id: "user-123",              // User ID
    email: "user@example.com",   // Email
    username: "johndoe",         // Username
    avatar: "https://example.com/avatar.jpg" // Avatar URL
)

// צור אסימון SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // העבר אסימון זה לממשק הקדמי שלך לאימות
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### SSO פשוט (לפיתוח/בדיקה)

```swift
import FastCommentsSwift

// צור נתוני משתמש SSO פשוטים (אין צורך במפתח API)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// צור אסימון SSO פשוט
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```