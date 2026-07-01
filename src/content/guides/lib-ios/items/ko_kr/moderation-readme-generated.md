### 모든 사용자가 사용할 수 있는 행동

- **Flag/Unflag** -- 댓글을 검토하기 위해 신고합니다

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- 사용자의 모든 댓글을 숨깁니다 (조회자별)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### 관리자 전용 행동

- **Pin/Unpin** -- 댓글을 스레드 상단에 고정합니다

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- 새로운 답글을 방지하고, 잠금 해제될 때까지 편집 및 삭제를 차단합니다 (모든 사용자, 관리자 포함)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

모든 관리 작업은 UI의 댓글 컨텍스트 메뉴에서도 사용할 수 있습니다. 관리 작업은 현재 사용자가 사이트 관리자일 때만 표시됩니다 (SSO `isAdmin` 플래그 또는 대시보드 설정을 통해 지정).