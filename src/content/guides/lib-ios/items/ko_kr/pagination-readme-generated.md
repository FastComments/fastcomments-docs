### 페이지 크기

```swift
// 댓글: 기본값 30
sdk.pageSize = 50

// 피드: 기본값 10
feedSDK.pageSize = 20
```

### 더 많은 댓글 불러오기

UI가 페이지네이션 컨트롤을 자동으로 표시합니다. 프로그래밍 방식으로도 페이지네이션을 트리거할 수 있습니다:

```swift
// 다음 페이지 불러오기
try await sdk.loadMore()

// 나머지 전체 불러오기 (성능상의 이유로 댓글이 2000개를 초과하면 비활성화됨)
try await sdk.loadAll()

// 상태 확인
sdk.hasMore            // 더 불러올 페이지가 있는지 여부
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### 하위 댓글 페이지네이션

중첩된 답글은 지연 로딩됩니다. 사용자가 스레드를 펼치면 처음 5개의 하위 댓글이 로드됩니다. 더 많은 답글이 있으면 "답글 더 보기" 컨트롤이 나타납니다. 이는 UI에서 자동으로 처리됩니다.