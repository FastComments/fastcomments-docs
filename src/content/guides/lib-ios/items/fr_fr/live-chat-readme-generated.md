`LiveChatView` fournit une expérience de chat en temps réel avec défilement automatique, séparateurs de date et une mise en page compacte. Il configure automatiquement le SDK pour un tri du plus ancien au plus récent et un affichage live immédiat.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO recommandé afin que les utilisateurs aient des noms
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

`LiveChatView` prend en charge ces callbacks :

- `.onCommentPosted` -- déclenché lorsque l'utilisateur envoie un message
- `.onCommentDeleted` -- déclenché lorsqu'un message est supprimé
- `.onUserClick` -- déclenché lorsque le nom ou l'avatar d'un utilisateur est touché

---
---