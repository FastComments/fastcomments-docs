`LiveChatView` מספק חוויית צ'אט בזמן אמת עם גלילה אוטומטית, מפרידי תאריכים וממשק קומפקטי. הוא מגדיר אוטומטית את ה-SDK למיון מהעתיק לחדש ותצוגה חיה מיידית.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // מומלץ להשתמש ב-SSO כדי שלמשתמשים יהיו שמות
        )
        return FastCommentsSDK(config: config)
    }()

    var body: some View {
        LiveChatView(sdk: sdk)
            .onCommentPosted { comment in
                print("Sent: \(comment.commentHTML)")
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

`LiveChatView` תומך בקריאות חזרה אלה:

- `.onCommentPosted` -- נקרא כאשר המשתמש שולח הודעה
- `.onCommentDeleted` -- נקרא כאשר הודעה נמחקת
- `.onUserClick` -- נקרא כאשר נלחץ שם המשתמש או תמונת הפרופיל שלו

---
---