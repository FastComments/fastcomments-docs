`LiveChatView` пружа разговор у реалном времену са аутоматским скроловањем, раздвајачима датума и компактним изгледом. Аутоматски конфигурише SDK за сортирање са најстаријим прво и тренутни приказ уживо.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO се препоручује како би корисници имали имена
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

`LiveChatView` подржава следеће повратне позиве:

- `.onCommentPosted` -- покреће се када корисник пошаље поруку
- `.onCommentDeleted` -- покреће се када се порука избрише
- `.onUserClick` -- покреће се када је корисничко име или аватар додирнут(и)