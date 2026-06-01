| Shortcode | 描述 |
| --- | --- |
| `fastcomments` | 具有回覆、投票與 @提及 的分層評論系統 |
| `fastcomments-comment-count` | 單頁的評論數 |
| `fastcomments-comment-count-bulk` | 一次請求取得多個頁面的評論數（請參見 [批次評論計數](#bulk-comment-counts-readme-generated)） |
| `fastcomments-live-chat` | 即時聊天小工具 |
| `fastcomments-collab-chat` | 協作式內嵌評論（需要 `target`） |
| `fastcomments-image-chat` | 圖片註解評論（需要 `target`） |
| `fastcomments-recent-comments` | 全站近期評論 |
| `fastcomments-recent-discussions` | 近期活躍的討論串 |
| `fastcomments-reviews-summary` | 星級評價摘要 |
| `fastcomments-top-pages` | 討論最多的頁面 |
| `fastcomments-user-activity-feed` | 每位使用者的活動摘要（需要 `userId`） |

### 範例

內嵌於文字中的評論數：

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

即時聊天：

```text
\{{< fastcomments-live-chat >}}
```

協作聊天，透過 CSS 選擇器指定內容元素：

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

圖片聊天，透過 CSS 選擇器指定圖像元素：

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

評價摘要：

```text
\{{< fastcomments-reviews-summary >}}
```

使用者活動摘要：

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```