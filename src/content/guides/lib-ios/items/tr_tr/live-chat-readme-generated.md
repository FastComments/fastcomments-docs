`LiveChatView` otomatik kaydırma, tarih ayırıcıları ve kompakt bir düzen ile gerçek zamanlı bir sohbet deneyimi sunar. SDK'yı otomatik olarak en eski ilk sıralama ve anında canlı görüntüleme için yapılandırır.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO önerilir, böylece kullanıcıların isimleri görünür
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

`LiveChatView` şu geri çağırmaları destekler:

- `.onCommentPosted` -- kullanıcı bir mesaj gönderdiğinde tetiklenir
- `.onCommentDeleted` -- bir mesaj silindiğinde tetiklenir
- `.onUserClick` -- bir kullanıcının adına veya avatarına dokunulduğunda tetiklenir

---
---