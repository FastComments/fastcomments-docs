FastComments תומך בשלושה מצבי אימות:

1. **אנונימי** -- ללא אסימון SSO; משתמשים מקבלים זהויות מבוססות הפעלה
2. **SSO פשוט** -- אסימון בצד הלקוח להדגמות ובדיקות (לא מאובטח)
3. **SSO מאובטח** -- אסימון חתום בצד השרת לסביבת פרודקשן

### SSO פשוט

שימושי להדגמות ובדיקות מקומיות. כל אחד יכול להתחזות לכל משתמש עם SSO פשוט, ולכן אין להשתמש בו בפרודקשן.

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

- `id` -- מזהה המשתמש (ברירת מחדל היא האימייל אם לא הוגדר)
- `displayName` -- שם תצוגה נפרד
- `displayLabel` -- תווית מותאמת אישית המוצגת ליד השם (למשל "VIP")
- `websiteUrl` -- קישור על שם המשתמש
- `locale` -- קוד לוקאל
- `isProfileActivityPrivate` -- הסתרת פעילות הפרופיל (ברירת מחדל היא true)

### SSO מאובטח

בפרודקשן, הבקאנד שלך מייצר אסימון SSO חתום באמצעות סוד ה-API שלך. אפליקציית ה-iOS מביאה את האסימון הזה מהשרת שלך ומעבירה אותו לקונפיגורציה.

**בבקאנד שלך** (באמצעות ה-SDK של FastComments ל-Swift או בכל שפה אחרת):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// החזר את האסימון הזה לאפליקציית ה-iOS שלך דרך ה-API שלך
```

**באפליקציית ה-iOS שלך:**

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
            // הבא את האסימון מהבקאנד שלך
            let token = try? await fetchSSOTokenFromYourBackend()
            // צור קונפיגורציה חדשה עם האסימון, או הגדר אותו לפני הטעינה
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` תומך בשדות נוספים:

- `optedInNotifications` -- הסכמה להתראות אימייל
- `displayLabel` -- תווית מותאמת אישית
- `displayName` -- שם תצוגה
- `websiteUrl` -- כתובת אתר
- `groupIds` -- חברויות בקבוצות
- `isAdmin` -- הרשאות מנהל
- `isModerator` -- הרשאות מפקח
- `isProfileActivityPrivate` -- פרטיות הפרופיל

---
---