`LiveChatView` pruža iskustvo chata u realnom vremenu sa automatskim skrolovanjem, separatorima po datumima i kompaktnim izgledom. Automatski konfiguriše SDK za sortiranje od najstarijih i trenutno prikazivanje uživo.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // Preporučeno je korišćenje SSO kako bi korisnici imali imena
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

`LiveChatView` podržava sledeće povratne pozive:

- `.onCommentPosted` -- se poziva kada korisnik pošalje poruku
- `.onCommentDeleted` -- se poziva kada je poruka obrisana
- `.onUserClick` -- se poziva kada se klikne ime ili avatar korisnika

---
---