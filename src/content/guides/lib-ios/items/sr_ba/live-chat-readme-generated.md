`LiveChatView` pruža chat u stvarnom vremenu sa automatskim skrolovanjem, separatorima datuma i kompaktim rasporedom. Automatski konfiguriše SDK za sortiranje po najstarijem (oldest-first) i za neposredno prikazivanje uživo.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO se preporučuje kako bi korisnici imali imena
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

`LiveChatView` podržava ove povratne pozive (callbacks):

- `.onCommentPosted` -- poziva se kada korisnik pošalje poruku
- `.onCommentDeleted` -- poziva se kada je poruka obrisana
- `.onUserClick` -- poziva se kada se klikne na korisničko ime ili avatar

---
---