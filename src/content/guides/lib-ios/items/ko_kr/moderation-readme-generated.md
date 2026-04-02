### 모든 사용자에게 제공되는 작업

- **신고/신고 취소** -- 댓글을 검토를 위해 신고

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **차단/차단 해제** -- 특정 사용자의 모든 댓글을 숨김 (뷰어별)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### 관리자 전용 작업

- **고정/고정 해제** -- 스레드 상단에 댓글 고정

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **잠금/잠금 해제** -- 댓글에 새 답글 작성 방지

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

모든 중재 작업은 UI의 댓글 컨텍스트 메뉴에서도 사용할 수 있습니다. 관리자 작업은 현재 사용자가 사이트 관리자일 때만 표시됩니다(SSO의 `isAdmin` 플래그 또는 대시보드 구성으로 설정).

---
---