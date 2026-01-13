### שימוש ב-API הציבורי

```swift
import FastCommentsSwift

// צור לקוח API
let publicApi = PublicAPI()

// שלוף תגובות עבור דף
do {
    let response = try await publicApi.getCommentsPublic(
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

### שימוש ב-API המאומת

```swift
import FastCommentsSwift

// צור תצורה עם מפתח ה-API
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// שלוף תגובות באמצעות ה-API המאומת
do {
    let response = try await defaultApi.getComments(
        tenantId: "your-tenant-id",
        urlId: "page-url-id"
    )

    print("Total comments: \(response.count ?? 0)")
    for comment in response.comments ?? [] {
        print("Comment ID: \(comment.id ?? ""), Text: \(comment.comment ?? "")")
    }
} catch {
    print("Error: \(error)")
}
```

### שימוש ב-SSO לאימות

#### SSO מאובטח (מומלץ בסביבת ייצור)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// צור נתוני משתמש SSO מאובטחים (צד שרת בלבד!)
let userData = SecureSSOUserData(
    id: "user-123",              // מזהה משתמש
    email: "user@example.com",   // דוא"ל
    username: "johndoe",         // שם משתמש
    avatar: "https://example.com/avatar.jpg" // כתובת ה-URL של האווטאר
)

// צור אסימון SSO
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // העבר אסימון זה לצד הלקוח שלך לצורכי אימות
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### SSO פשוט (לפיתוח/בדיקות)

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