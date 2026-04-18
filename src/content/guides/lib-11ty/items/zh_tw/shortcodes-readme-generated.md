---
| 短碼 | 描述 |
| --- | --- |
| `fastcomments` | 具有回覆、投票等功能的留言小工具 |
| `fastcommentsCommentCount` | 顯示頁面留言數 |
| `fastcommentsImageChat` | 圖片註解留言 |
| `fastcommentsLiveChat` | 即時聊天小工具 |
| `fastcommentsCollabChat` | 協作內聯留言 |
| `fastcommentsRecentComments` | 整站近期留言 |
| `fastcommentsRecentDiscussions` | 最近活躍的討論串 |
| `fastcommentsReviewsSummary` | 星級評分評論摘要 |
| `fastcommentsTopPages` | 最多討論的頁面 |
| `fastcommentsUserActivityFeed` | 使用者活動動態 |

### 範例

```njk
{# 內聯顯示留言計數 #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# 即時聊天 #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# 協作聊天 — 以 CSS 選擇器指定目標內容元素 #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# 圖片聊天 — 以 CSS 選擇器指定目標圖像元素 #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# 評論摘要 #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# 使用者活動動態 #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```
---