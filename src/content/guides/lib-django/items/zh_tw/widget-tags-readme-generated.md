每個小工具都有自己的標籤。所有標籤皆接受 `**extra` 關鍵字參數，這些參數會以原樣（使用 camelCase 鍵）合併至小工具設定中，針對以下未列出的情況。

| 標籤 | 小工具 |
|---|---|
| `{% fastcomments %}` | 評論 |
| `{% fastcomments_live_chat %}` | 即時聊天 |
| `{% fastcomments_comment_count %}` | 評論數量徽章 |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | 批量評論計數 |
| `{% fastcomments_collab_chat target="#el" %}` | 協作（內嵌）聊天 |
| `{% fastcomments_image_chat target="#el" %}` | 圖片批註聊天 |
| `{% fastcomments_recent_comments %}` | 最近評論 |
| `{% fastcomments_recent_discussions %}` | 最近討論 |
| `{% fastcomments_reviews_summary %}` | 評論摘要 |
| `{% fastcomments_top_pages %}` | 最受討論頁面 |
| `{% fastcomments_user_activity user_id="..." %}` | 使用者的活動資訊流 |

具名參數對應小工具的 camelCase 設定鍵：

| 參數 | 設定鍵 | 標籤 |
|---|---|---|
| `url_id` | `urlId` | 評論, 即時聊天, 評論數量, 協作/圖片聊天, 最近評論, 評論摘要 |
| `url` | `url` | 評論, 即時聊天, 協作/圖片聊天 |
| `readonly` | `readonly` | 評論, 即時聊天, 協作/圖片聊天 |
| `locale` | `locale` | 評論, 即時聊天, 協作/圖片聊天, 使用者活動 |
| `has_dark_background` | `hasDarkBackground` | 全部 |
| `default_sort_direction` | `defaultSortDirection` | 評論, 即時聊天, 協作/圖片聊天 |
| `number_only` | `numberOnly` | 評論數量 |
| `is_live` | `isLive` | 評論數量 |
| `count` | `count` | 最近評論, 最近討論 |
| `target` | (querySelector, not sent) | 協作聊天, 圖片聊天 |
| `chat_square_percentage` | `chatSquarePercentage` | 圖片聊天 |
| `user_id` | `userId` | 使用者活動 |

範例：

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

評論： {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# 協作聊天附加於頁面上已存在的元素 #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# 批量計數：放置標記，之後一次批量載入器會填入所有數值 #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```