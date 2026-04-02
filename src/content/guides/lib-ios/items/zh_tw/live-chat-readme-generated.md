`LiveChatView` 提供即時聊天體驗，具自動滾動、日期分隔符和緊湊的佈局。它會自動將 SDK 設定為由最舊至最新排序並立即顯示實況。

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // 建議使用 SSO，讓使用者有名稱
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

`LiveChatView` 支援以下回呼：

- `.onCommentPosted` -- 在使用者發送訊息時觸發
- `.onCommentDeleted` -- 當訊息被刪除時觸發
- `.onUserClick` -- 當使用者的名稱或頭像被點擊時觸發

---
---