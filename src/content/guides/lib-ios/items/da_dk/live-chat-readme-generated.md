`LiveChatView` giver en realtids chatoplevelse med automatisk rulning, datoseparatorer og et kompakt layout. Den konfigurerer SDK'en automatisk til ældste-først sortering og øjeblikkelig live-visning.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO anbefales, så brugerne har navne
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

- `.onCommentPosted` -- udløses, når brugeren sender en besked
- `.onCommentDeleted` -- udløses, når en besked slettes
- `.onUserClick` -- udløses, når en brugers navn eller avatar trykkes på

---
---