`LiveChatView` bietet eine Echtzeit-Chat-Erfahrung mit automatischem Scrollen, Datums-Trennzeichen und einem kompakten Layout. Es konfiguriert das SDK automatisch für eine Sortierung "älteste zuerst" und sofortige Live-Anzeige.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO empfohlen, damit Benutzer Namen haben
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

`LiveChatView` unterstützt diese Callbacks:

- `.onCommentPosted` -- wird ausgelöst, wenn der Benutzer eine Nachricht sendet
- `.onCommentDeleted` -- wird ausgelöst, wenn eine Nachricht gelöscht wird
- `.onUserClick` -- wird ausgelöst, wenn auf den Namen oder das Avatar eines Benutzers getippt wird

---
---