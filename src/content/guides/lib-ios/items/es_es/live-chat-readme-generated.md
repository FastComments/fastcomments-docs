`LiveChatView` ofrece una experiencia de chat en tiempo real con desplazamiento automático, separadores de fecha y un diseño compacto. Configura automáticamente el SDK para ordenación de más antiguos primero y visualización en vivo inmediata.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // Se recomienda SSO para que los usuarios tengan nombres
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

- `.onCommentPosted` -- se activa cuando el usuario envía un mensaje
- `.onCommentDeleted` -- se activa cuando se elimina un mensaje
- `.onUserClick` -- se activa cuando se pulsa el nombre o el avatar de un usuario

---
---