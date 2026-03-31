`LiveChatView` пружа искуство разговора у реалном времену са аутоматским скроловањем, раздвајачима по датумима и компактним распоредом. Аутоматски конфигурише SDK за сортирање најстаријих прво и одмах приказ уживо.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO се препоручује тако да корисници имају имена
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

`LiveChatView` подржава ове повратне позиве:

- `.onCommentPosted` -- активира се када корисник пошаље поруку
- `.onCommentDeleted` -- активира се када се порука обрише
- `.onUserClick` -- активира се када се кликне на име корисника или аватар

---
---