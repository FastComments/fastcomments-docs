Every widget has its own tag. All of them accept `**extra` keyword arguments,
이는 위젯 구성에 그대로 병합되며 (camelCase 키 사용) 아래에 명시된 명명된 인수로
다루어지지 않는 모든 경우에 적용됩니다.

| 태그 | 위젯 |
|---|---|
| `{% fastcomments %}` | 댓글 |
| `{% fastcomments_live_chat %}` | 실시간 채팅 |
| `{% fastcomments_comment_count %}` | 댓글 수 배지 |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | 일괄 댓글 수 |
| `{% fastcomments_collab_chat target="#el" %}` | 협업(인라인) 채팅 |
| `{% fastcomments_image_chat target="#el" %}` | 이미지 주석 채팅 |
| `{% fastcomments_recent_comments %}` | 최근 댓글 |
| `{% fastcomments_recent_discussions %}` | 최근 토론 |
| `{% fastcomments_reviews_summary %}` | 리뷰 요약 |
| `{% fastcomments_top_pages %}` | 가장 많이 논의된 페이지 |
| `{% fastcomments_user_activity user_id="..." %}` | 사용자 활동 피드 |

명명된 인수는 위젯의 camelCase 구성 키에 매핑됩니다:

| 인수 | 구성 키 | 태그 |
|---|---|---|
| `url_id` | `urlId` | 댓글, 실시간 채팅, 댓글 수, 협업/이미지 채팅, 최근 댓글, 리뷰 요약 |
| `url` | `url` | 댓글, 실시간 채팅, 협업/이미지 채팅 |
| `readonly` | `readonly` | 댓글, 실시간 채팅, 협업/이미지 채팅 |
| `locale` | `locale` | 댓글, 실시간 채팅, 협업/이미지 채팅, 사용자 활동 |
| `has_dark_background` | `hasDarkBackground` | 모두 |
| `default_sort_direction` | `defaultSortDirection` | 댓글, 실시간 채팅, 협업/이미지 채팅 |
| `number_only` | `numberOnly` | 댓글 수 |
| `is_live` | `isLive` | 댓글 수 |
| `count` | `count` | 최근 댓글, 최근 토론 |
| `target` | (querySelector, not sent) | 협업 채팅, 이미지 채팅 |
| `chat_square_percentage` | `chatSquarePercentage` | 이미지 채팅 |
| `user_id` | `userId` | 사용자 활동 |

예시:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# 협업 채팅이 페이지의 기존 요소에 연결됩니다 #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# 일괄 카운트: 마커를 배치한 뒤 일괄 로더가 모두 채웁니다 #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```