`LiveChatView` fornece uma experiência de chat em tempo real com rolagem automática, separadores de data e um layout compacto. Ele configura automaticamente o SDK para ordenação do mais antigo para o mais recente e exibição ao vivo imediata.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO recomendado para que os usuários tenham nomes
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

`LiveChatView` suporta estes callbacks:

- `.onCommentPosted` -- disparado quando o usuário envia uma mensagem
- `.onCommentDeleted` -- disparado quando uma mensagem é excluída
- `.onUserClick` -- disparado quando o nome ou avatar de um usuário é tocado

---
---