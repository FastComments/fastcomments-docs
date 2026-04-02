`LiveChatView` biedt een realtime chatervaring met automatisch scrollen, datum-scheidingen en een compacte lay-out. Het configureert de SDK automatisch voor oudste-eerst sortering en directe live-weergave.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO aanbevolen zodat gebruikers namen hebben
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

`LiveChatView` ondersteunt deze callbacks:

- `.onCommentPosted` -- wordt aangeroepen wanneer de gebruiker een bericht verstuurt
- `.onCommentDeleted` -- wordt aangeroepen wanneer een bericht wordt verwijderd
- `.onUserClick` -- wordt aangeroepen wanneer op de naam of avatar van een gebruiker wordt getikt

---
---