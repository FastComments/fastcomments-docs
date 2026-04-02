FastComments תומך בשלושה מצבי אימות:

1. **Anonymous** -- ללא אסימון SSO; למשתמשים ניתנות זהויות מבוססות סשן
2. **Simple SSO** -- אסימון בצד הלקוח עבור הדגמות ובדיקות (לא מאובטח)
3. **Secure SSO** -- אסימון החתום על ידי השרת לשימוש בייצור

### Simple SSO

שימושי להדגמות ובדיקות מקומיות. כל אחד יכול להתחפש לכל משתמש עם Simple SSO, לכן אל תשתמשו בו בפרודקשן.

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

`SimpleSSOUserData` תומך גם בשדות אופציונליים:

- `id` -- מזהה משתמש (ברירת מחדל: דואר אלקטרוני אם לא מוגדר)
- `displayName` -- שם תצוגה נפרד
- `displayLabel` -- תווית מותאמת שמוצגת לצד השם (למשל "VIP")
- `websiteUrl` -- קישור על שם המשתמש
- `locale` -- קוד שפה/אזור
- `isProfileActivityPrivate` -- הסתרת פעילות הפרופיל (ברירת מחדל: true)

### Secure SSO

בפרודקשן, ה-backend שלכם מייצר אסימון SSO חתום באמצעות סוד ה-API שלכם. אפליקציית iOS מקבלת את האסימון מהשרת שלכם ומעבירה אותו לקונפיגורציה.

**בצד השרת שלכם** (באמצעות ה-FastComments Swift SDK או בכל שפה):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Return this token to your iOS app via your API
```

**ביישום ה-iOS שלכם:**

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
            // Fetch the token from your backend
            let token = try? await fetchSSOTokenFromYourBackend()
            // Create a new config with the token, or set it before load
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` תומך בשדות נוספים:

- `optedInNotifications` -- הסכמה לקבלת התראות בדואר אלקטרוני
- `displayLabel` -- תווית מותאמת
- `displayName` -- שם תצוגה
- `websiteUrl` -- כתובת האתר
- `groupIds` -- שייכות לקבוצות
- `isAdmin` -- הרשאות מנהל
- `isModerator` -- הרשאות מודרטור
- `isProfileActivityPrivate` -- פרטיות פעילות הפרופיל