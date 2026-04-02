`LiveChatView` fornisce un'esperienza di chat in tempo reale con scorrimento automatico, separatori di data e un layout compatto. Configura automaticamente l'SDK per l'ordinamento dal più vecchio al più recente e la visualizzazione live immediata.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO consigliato in modo che gli utenti abbiano nomi
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

`LiveChatView` supporta i seguenti callback:

- `.onCommentPosted` -- viene attivato quando l'utente invia un messaggio
- `.onCommentDeleted` -- viene attivato quando un messaggio viene eliminato
- `.onUserClick` -- viene attivato quando si tocca il nome o l'avatar di un utente

---
---