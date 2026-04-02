`LiveChatView`는 자동 스크롤, 날짜 구분선, 컴팩트 레이아웃을 갖춘 실시간 채팅 환경을 제공합니다. SDK를 가장 오래된 항목 우선 정렬 및 즉시 라이브 표시로 자동 구성합니다.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // 사용자의 이름 표시를 위해 SSO 사용을 권장합니다
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

`LiveChatView`는 다음 콜백을 지원합니다:

- `.onCommentPosted` -- 사용자가 메시지를 보낼 때 호출됩니다
- `.onCommentDeleted` -- 메시지가 삭제될 때 호출됩니다
- `.onUserClick` -- 사용자의 이름이나 아바타를 탭했을 때 호출됩니다

---
---