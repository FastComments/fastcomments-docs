---
來自 `fastcomments-react` 的每個元件都以相同的名稱提供：

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | 旗艦即時留言元件 |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | 內嵌評論計數徽章 |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | 串流式即時聊天元件 |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | 以文字為錨點的協作聊天 |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | 基於區域的圖片評論 |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | 近期評論動態 |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | 近期討論動態 |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | 星級評分摘要 |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | 最多評論頁面排行榜 |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | 每位使用者的活動時間線 |

### 附加到現有元素的元件

`FastCommentsCollabChatWidget` and `FastCommentsImageChatWidget` mount into a caller-supplied
element. Pass a `targetRef` accessor that returns the element once mounted:

```tsx
import { FastCommentsImageChatWidget } from 'fastcomments-solidjs';

export default function ImageChat() {
  let img: HTMLImageElement | undefined;
  return (
    <>
      <img ref={img} src="/screenshot.png" alt="" />
      <FastCommentsImageChatWidget
        tenantId="demo"
        urlId="my-image"
        targetRef={() => img}
      />
    </>
  );
}
```

### 區域

傳遞 `region="eu"` 以透過 EU 叢集引導元件流量。
---