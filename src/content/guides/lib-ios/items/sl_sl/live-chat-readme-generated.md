`LiveChatView` nudi izkušnjo klepeta v realnem času z avtomatskim pomikanjem, ločilci datumov in kompaktno postavitvijo. Samodejno konfigurira SDK za razvrščanje od najstarejših naprej in takojšnji prikaz v živo.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // Priporočamo SSO, da imajo uporabniki imena
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

`LiveChatView` podpira naslednje povratne klice:

- `.onCommentPosted` -- sproži se, ko uporabnik pošlje sporočilo
- `.onCommentDeleted` -- sproži se, ko je sporočilo izbrisano
- `.onUserClick` -- sproži se, ko je kliknjen uporabnikovo ime ali avatar

---
---