| Tag | Description |
| --- | --- |
| `fastcomments` | 即時留言功能，支援回覆、投票、審核與即時更新 |
| `fastcomments_comment_count` | 當前頁面的留言數量 |
| `fastcomments_comment_count_bulk` | 清單/索引頁面上多個頁面的留言數量 |
| `fastcomments_live_chat` | 即時串流聊天小工具 |
| `fastcomments_collab_chat` | 協作式內嵌留言（文字註解） |
| `fastcomments_image_chat` | 圖像註解留言 |
| `fastcomments_recent_comments` | 全站最近留言 |
| `fastcomments_recent_discussions` | 最近活躍的討論串 |
| `fastcomments_reviews_summary` | 星級評分評論摘要 |
| `fastcomments_top_pages` | 討論最多的頁面 |
| `fastcomments_user_activity_feed` | 每位使用者的活動動態 |

### 範例

```liquid
{% raw %}{# 留言數量。該小工具會自行呈現標籤，例如「0 則留言」 #}
{% fastcomments_comment_count %}

{# 即時聊天 #}
{% fastcomments_live_chat %}

{# 協作聊天。指定一個帶 CSS 選擇器的內容元素 #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# 圖像聊天。指定一個帶 CSS 選擇器的圖像元素 #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# 評論摘要 #}
{% fastcomments_reviews_summary %}

{# 使用者活動動態。需要 user_id #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# 部落格索引的批量留言計數 #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```