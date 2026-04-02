### 편집

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

서버가 HTML을 다시 렌더링합니다. 로컬 댓글이 자동으로 업데이트됩니다.

### 삭제

```swift
try await sdk.deleteComment(commentId: commentId)
```

댓글을 삭제하면 그 댓글의 하위 댓글들도 로컬 트리에서 함께 제거됩니다.

두 작업 모두 현재 사용자가 댓글 작성자(또는 사이트 관리자)인 경우 UI의 댓글 컨텍스트 메뉴를 통해 이용할 수 있습니다.

---
---