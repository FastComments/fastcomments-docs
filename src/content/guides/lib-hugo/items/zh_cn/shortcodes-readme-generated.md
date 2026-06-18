| Shortcode | 描述 |
| --- | --- |
| `fastcomments` | 带回复、投票和@提及的线程式评论 |
| `fastcomments-comment-count` | 单页评论计数 |
| `fastcomments-comment-count-bulk` | 一次请求获取多个页面的评论计数（参见 [批量评论计数](#bulk-comment-counts-readme-generated)） |
| `fastcomments-live-chat` | 实时聊天小部件 |
| `fastcomments-collab-chat` | 协作内联评论（需要 `target`） |
| `fastcomments-image-chat` | 图像注释评论（需要 `target`） |
| `fastcomments-recent-comments` | 全站最新评论 |
| `fastcomments-recent-discussions` | 最近活跃的讨论线程 |
| `fastcomments-reviews-summary` | 星级评分评论摘要 |
| `fastcomments-top-pages` | 讨论最多的页面 |
| `fastcomments-user-activity-feed` | 每用户活动提要（需要 `userId`） |

### 示例

文本内联的评论计数：

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

实时聊天：

```text
\{{< fastcomments-live-chat >}}
```

协作聊天，通过 CSS 选择器定位内容元素：

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

图片聊天，通过 CSS 选择器定位图像元素：

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

评论摘要：

```text
\{{< fastcomments-reviews-summary >}}
```

用户活动提要：

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```