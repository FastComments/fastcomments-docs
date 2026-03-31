`LiveChatView` 提供实时聊天体验，带有自动滚动、日期分隔符和紧凑布局。它会自动将 SDK 配置为按最早优先排序并立即以实时方式显示。

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // 建议使用 SSO，这样用户会有名称
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

`LiveChatView` 支持以下回调：

- `.onCommentPosted` -- 当用户发送消息时触发
- `.onCommentDeleted` -- 当消息被删除时触发
- `.onUserClick` -- 当用户的名字或头像被点击时触发