---
| Shortcode | Description |
| --- | --- |
| `fastcomments` | 답글, 투표, 그리고 @멘션을 지원하는 스레드형 댓글 |
| `fastcomments-comment-count` | 단일 페이지의 댓글 수 |
| `fastcomments-comment-count-bulk` | 한 요청으로 여러 페이지의 댓글 수 (자세한 내용은 [대량 댓글 수](#bulk-comment-counts-readme-generated) 참조) |
| `fastcomments-live-chat` | 실시간 채팅 위젯 |
| `fastcomments-collab-chat` | 협업 인라인 주석(사용하려면 `target` 필요) |
| `fastcomments-image-chat` | 이미지 주석 댓글(사용하려면 `target` 필요) |
| `fastcomments-recent-comments` | 사이트 전반의 최근 댓글 |
| `fastcomments-recent-discussions` | 최근에 활동한 토론 스레드 |
| `fastcomments-reviews-summary` | 별점 리뷰 요약 |
| `fastcomments-top-pages` | 가장 많이 논의된 페이지 |
| `fastcomments-user-activity-feed` | 사용자별 활동 피드 (사용하려면 `userId` 필요) |

### 예제

텍스트 내 인라인 댓글 수:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

라이브 채팅:

```text
\{{< fastcomments-live-chat >}}
```

협업 채팅, CSS 선택자로 콘텐츠 요소를 지정:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

이미지 채팅, 이미지 요소를 CSS 선택자로 지정:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

리뷰 요약:

```text
\{{< fastcomments-reviews-summary >}}
```

사용자 활동 피드:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```
---