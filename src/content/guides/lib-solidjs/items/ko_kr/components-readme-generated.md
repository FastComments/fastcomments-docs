`fastcomments-react`의 모든 위젯은 동일한 이름으로 사용 가능합니다:

| 컴포넌트 | 핸들 타입 | 로드된 임베드 |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | 주력 실시간 댓글 위젯 |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | 인라인 댓글 수 배지 |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | 스트리밍 라이브 채팅 위젯 |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | 텍스트 앵커 기반 협업 채팅 |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | 영역 기반 이미지 댓글 |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | 최근 댓글 피드 |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | 최근 토론 피드 |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | 별점 요약 |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | 댓글 최다 페이지 순위 |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | 사용자별 활동 타임라인 |

### 기존 요소에 첨부되는 위젯

`FastCommentsCollabChatWidget`와 `FastCommentsImageChatWidget`은 호출자가 제공한 요소에 마운트됩니다. 마운트된 후 요소를 반환하는 `targetRef` 접근자를 전달하세요:

```tsx
import { FastCommentsImageChatWidget } from 'fastcomments-solidjs';

export default function ImageChat() {
  let img: HTMLImageElement | undefined;
  return (
    <>
      <img ref={img} src="/screenshot.png" alt="" />
      <FastCommentsImageChatWidget
        tenantId="demo"
        urlId="my-image"
        targetRef={() => img}
      />
    </>
  );
}
```

### 리전

`region="eu"`를 전달하면 위젯 트래픽이 EU 클러스터를 통해 라우팅됩니다.