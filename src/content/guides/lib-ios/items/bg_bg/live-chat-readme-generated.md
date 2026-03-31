`LiveChatView` предоставя чат в реално време с автоматично превъртане, разделители по дати и компактен изглед. Той автоматично конфигурира SDK за сортиране от най-старите към най-новите и незабавно показване на живо.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // Препоръчва се SSO, за да имат потребителите имена
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

`LiveChatView` supports these callbacks:

- `.onCommentPosted` -- извиква се, когато потребителят изпрати съобщение
- `.onCommentDeleted` -- извиква се, когато съобщение бъде изтрито
- `.onUserClick` -- извиква се, когато името или аватарът на потребител бъде докоснат

---
---