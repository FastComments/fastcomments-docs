每个小部件都有其专属标签。所有标签都接受 `**extra` 关键字参数，这些参数会原样合并到小部件的配置中（使用 camelCase 键），用于处理下面未列出的命名参数。

| 标签 | 小部件 |
|---|---|
| `{% fastcomments %}` | 评论 |
| `{% fastcomments_live_chat %}` | 实时聊天 |
| `{% fastcomments_comment_count %}` | 评论计数徽章 |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | 批量评论计数 |
| `{% fastcomments_collab_chat target="#el" %}` | 协作（内联）聊天 |
| `{% fastcomments_image_chat target="#el" %}` | 图片标注聊天 |
| `{% fastcomments_recent_comments %}` | 最近评论 |
| `{% fastcomments_recent_discussions %}` | 最近讨论 |
| `{% fastcomments_reviews_summary %}` | 评论摘要 |
| `{% fastcomments_top_pages %}` | 讨论最多的页面 |
| `{% fastcomments_user_activity user_id="..." %}` | 用户活动流 |

命名参数对应小部件的 camelCase 配置键：

| 参数 | 配置键 | 标签 |
|---|---|---|
| `url_id` | `urlId` | 评论、实时聊天、评论计数、协作/图片聊天、最近评论、评论摘要 |
| `url` | `url` | 评论、实时聊天、协作/图片聊天 |
| `readonly` | `readonly` | 评论、实时聊天、协作/图片聊天 |
| `locale` | `locale` | 评论、实时聊天、协作/图片聊天、用户活动 |
| `has_dark_background` | `hasDarkBackground` | 所有 |
| `default_sort_direction` | `defaultSortDirection` | 评论、实时聊天、协作/图片聊天 |
| `number_only` | `numberOnly` | 评论计数 |
| `is_live` | `isLive` | 评论计数 |
| `count` | `count` | 最近评论、最近讨论 |
| `target` | (querySelector, not sent) | 协作聊天、图片聊天 |
| `chat_square_percentage` | `chatSquarePercentage` | 图片聊天 |
| `user_id` | `userId` | 用户活动 |

示例：

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# 协作聊天附加到页面上已有的元素 #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# 批量计数：放置标记，然后一个批量加载器一次性填充所有计数 #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```