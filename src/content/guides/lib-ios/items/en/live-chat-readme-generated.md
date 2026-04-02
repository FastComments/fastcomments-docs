`LiveChatView` provides a real-time chat experience with auto-scroll, date separators, and a compact layout. It automatically configures the SDK for oldest-first sort and immediate live display.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO recommended so users have names
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

- `.onCommentPosted` -- fired when the user sends a message
- `.onCommentDeleted` -- fired when a message is deleted
- `.onUserClick` -- fired when a user's name or avatar is tapped

---