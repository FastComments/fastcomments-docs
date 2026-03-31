`LiveChatView`は自動スクロール、日付区切り、コンパクトなレイアウトを備えたリアルタイムチャット体験を提供します。SDKを古い順（oldest-first）のソートと即時ライブ表示用に自動的に構成します。

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // ユーザーに名前が表示されるよう、SSOの使用を推奨します
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

`LiveChatView`は次のコールバックをサポートします:

- `.onCommentPosted` -- ユーザーがメッセージを送信したときに呼び出されます
- `.onCommentDeleted` -- メッセージが削除されたときに呼び出されます
- `.onUserClick` -- ユーザーの名前やアバターがタップされたときに呼び出されます

---
---