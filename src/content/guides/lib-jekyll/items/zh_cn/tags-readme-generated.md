| 标签 | 描述 |
| --- | --- |
| `fastcomments` | 实时评论，支持回复、投票、审核和实时更新 |
| `fastcomments_comment_count` | 当前页面的评论数量 |
| `fastcomments_comment_count_bulk` | 用于列表/索引页上多个页面的评论数量 |
| `fastcomments_live_chat` | 实时流式聊天小部件 |
| `fastcomments_collab_chat` | 协作式内联评论（文本注释） |
| `fastcomments_image_chat` | 图像注释评论 |
| `fastcomments_recent_comments` | 整个站点的最新评论 |
| `fastcomments_recent_discussions` | 最近活跃的讨论线程 |
| `fastcomments_reviews_summary` | 星级评分汇总 |
| `fastcomments_top_pages` | 讨论最多的页面 |
| `fastcomments_user_activity_feed` | 每个用户的活动动态 |

### 示例

```liquid
{% raw %}{# 评论数量。该小部件会自行渲染标签，例如 "0 comments" #}
{% fastcomments_comment_count %}

{# 实时聊天 #}
{% fastcomments_live_chat %}

{# 协作聊天。通过 CSS 选择器将其指向某个内容元素 #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# 图像聊天。通过 CSS 选择器将其指向某个图像元素 #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# 评价摘要 #}
{% fastcomments_reviews_summary %}

{# 用户活动动态。需要一个用户 ID #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# 博客索引页的批量评论计数 #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```