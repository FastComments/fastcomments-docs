### Page Size

```swift
// 댓글: 기본값 30
sdk.pageSize = 50

// 피드: 기본값 10
feedSDK.pageSize = 20
```

### Loading More Comments

The UI shows pagination controls automatically. You can also trigger pagination programmatically:

```swift
// 다음 페이지 로드
try await sdk.loadMore()

// 모든 남은 항목 로드 (성능 상 댓글이 2000개 초과인 경우 비활성화됨)
try await sdk.loadAll()

// 상태 확인
sdk.hasMore            // 더 많은 페이지가 있는지
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Child Comment Pagination

중첩된 답글은 지연 로드됩니다. 사용자가 스레드를 확장하면 처음 5개의 자식이 로드됩니다. 더 많은 항목이 있으면 "더 많은 답글 불러오기" 컨트롤이 나타납니다. 이 동작은 UI에서 자동으로 처리됩니다.

---
---