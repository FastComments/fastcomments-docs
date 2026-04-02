`LiveChatView` pruža iskustvo chata u stvarnom vremenu s automatskim pomicanjem, razdjelnicima datuma i kompaktim izgledom. Automatski konfigurira SDK za sortiranje od najstarijih prema najnovijima i trenutni prikaz uživo.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // Preporučeno SSO kako bi korisnici imali imena
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

`LiveChatView` podržava ove povratne pozive:

- `.onCommentPosted` -- pozvano kada korisnik pošalje poruku
- `.onCommentDeleted` -- pozvano kada se poruka izbriše
- `.onUserClick` -- pozvano kada se dodirne ime ili avatar korisnika