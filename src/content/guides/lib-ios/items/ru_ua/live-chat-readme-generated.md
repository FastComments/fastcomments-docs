`LiveChatView` предоставляет чат в реальном времени с автопрокруткой, разделителями по дате и компактным макетом. Он автоматически настраивает SDK для сортировки от самых старых и немедленного отображения в режиме реального времени.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // Рекомендуется SSO, чтобы у пользователей были имена
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

`LiveChatView` поддерживает следующие обратные вызовы:

- `.onCommentPosted` -- срабатывает, когда пользователь отправляет сообщение
- `.onCommentDeleted` -- срабатывает, когда сообщение удаляется
- `.onUserClick` -- срабатывает, когда нажимают на имя пользователя или его аватар

---
---