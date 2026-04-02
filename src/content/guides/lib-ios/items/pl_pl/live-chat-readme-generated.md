`LiveChatView` zapewnia doświadczenie czatu w czasie rzeczywistym z automatycznym przewijaniem, separatorami dat i kompaktowym układem. Automatycznie konfiguruje SDK do sortowania od najstarszych oraz natychmiastowego wyświetlania na żywo.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO zalecane, aby użytkownicy mieli nazwy
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

`LiveChatView` obsługuje te wywołania zwrotne:

- `.onCommentPosted` -- wywoływane, gdy użytkownik wysyła wiadomość
- `.onCommentDeleted` -- wywoływane, gdy wiadomość zostanie usunięta
- `.onUserClick` -- wywoływane, gdy kliknięto nazwę użytkownika lub awatar

---
---