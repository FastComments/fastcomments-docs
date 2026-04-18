| Shortcode | Description |
| --- | --- |
| `fastcomments` | 带回复、投票等功能的评论小部件 |
| `fastcommentsCommentCount` | 显示页面的评论计数 |
| `fastcommentsImageChat` | 图片标注评论 |
| `fastcommentsLiveChat` | 实时聊天小部件 |
| `fastcommentsCollabChat` | 协作式内嵌评论 |
| `fastcommentsRecentComments` | 站点范围内的最新评论 |
| `fastcommentsRecentDiscussions` | 最近活跃的讨论线程 |
| `fastcommentsReviewsSummary` | 星级评分评论摘要 |
| `fastcommentsTopPages` | 讨论最多的页面 |
| `fastcommentsUserActivityFeed` | 用户活动动态 |

### 示例

```njk
{# 内联显示评论数量 #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# 实时聊天 #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# 协作聊天 — 通过 CSS 选择器定位目标内容元素 #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# 图片聊天 — 通过 CSS 选择器定位目标图像元素 #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# 评论摘要 #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# 用户活动动态 #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```