`LiveChatView` забезпечує чат у реальному часі з автопрокруткою, розділювачами дат та компактним макетом. Він автоматично конфігурує SDK для сортування від найстаріших і негайного відображення в реальному часі.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // Рекомендовано SSO, щоб у користувачів були імена
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

`LiveChatView` підтримує ці зворотні виклики:

- `.onCommentPosted` -- викликається, коли користувач надсилає повідомлення
- `.onCommentDeleted` -- викликається, коли повідомлення видаляється
- `.onUserClick` -- викликається, коли натискають на ім'я або аватар користувача

---
---