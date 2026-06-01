| Tag | 설명 |
| --- | --- |
| `fastcomments` | 답글, 투표, 중재 및 실시간 업데이트가 가능한 라이브 댓글 |
| `fastcomments_comment_count` | 현재 페이지의 댓글 수 |
| `fastcomments_comment_count_bulk` | 목록/인덱스 페이지에서 여러 페이지의 댓글 수 |
| `fastcomments_live_chat` | 실시간 스트리밍 채팅 위젯 |
| `fastcomments_collab_chat` | 협업형 인라인 댓글(텍스트 주석) |
| `fastcomments_image_chat` | 이미지 주석 댓글 |
| `fastcomments_recent_comments` | 사이트 전체의 최근 댓글 |
| `fastcomments_recent_discussions` | 최근에 활성화된 토론 스레드 |
| `fastcomments_reviews_summary` | 별점 리뷰 요약 |
| `fastcomments_top_pages` | 가장 많이 논의된 페이지 |
| `fastcomments_user_activity_feed` | 사용자별 활동 피드 |

### 예제

```liquid
{% raw %}{# 댓글 수. 위젯이 자체 레이블을 렌더링합니다. 예: "0개의 댓글" #}
{% fastcomments_comment_count %}

{# 실시간 채팅 #}
{% fastcomments_live_chat %}

{# 협업 채팅. CSS 선택자로 콘텐츠 요소를 지정하세요 #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# 이미지 채팅. CSS 선택자로 이미지 요소를 지정하세요 #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# 리뷰 요약 #}
{% fastcomments_reviews_summary %}

{# 사용자 활동 피드. 사용자 id가 필요합니다 #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# 블로그 인덱스용 다중 댓글 수 집계 #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```