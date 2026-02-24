---
| 구성 요소 | 설명 |
| --- | --- |
| `FastComments` | 답글과 투표 등 기능을 갖춘 댓글 위젯 |
| `FastCommentsCommentCount` | 페이지의 댓글 수 표시 |
| `FastCommentsImageChat` | 이미지 주석 댓글 |
| `FastCommentsLiveChat` | 라이브 채팅 위젯 |
| `FastCommentsCollabChat` | 협업 인라인 댓글 |
| `FastCommentsReviewsSummary` | 별점 리뷰 요약 |
| `FastCommentsUserActivityFeed` | 사용자 활동 피드 |

모든 컴포넌트는 패키지 루트에서 내보내집니다:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---