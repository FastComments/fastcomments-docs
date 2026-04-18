| 숏코드 | 설명 |
| --- | --- |
| `fastcomments` | 답글, 투표 등 다양한 기능을 갖춘 댓글 위젯 |
| `fastcommentsCommentCount` | 페이지의 댓글 수를 표시합니다. |
| `fastcommentsImageChat` | 이미지 주석 댓글 |
| `fastcommentsLiveChat` | 라이브 채팅 위젯 |
| `fastcommentsCollabChat` | 협업 인라인 댓글 |
| `fastcommentsRecentComments` | 사이트 전체의 최근 댓글 |
| `fastcommentsRecentDiscussions` | 최근 활성 토론 스레드 |
| `fastcommentsReviewsSummary` | 별점 기반 리뷰 요약 |
| `fastcommentsTopPages` | 가장 많이 논의된 페이지 |
| `fastcommentsUserActivityFeed` | 사용자 활동 피드 |

### 예제

```njk
{# 텍스트와 함께 인라인으로 표시되는 댓글 수 #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# 라이브 채팅 #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# 협업 채팅 — CSS 선택자로 콘텐츠 요소를 대상으로 지정 #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# 이미지 채팅 — CSS 선택자로 이미지 요소를 대상으로 지정 #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# 리뷰 요약 #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# 사용자 활동 피드 #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```