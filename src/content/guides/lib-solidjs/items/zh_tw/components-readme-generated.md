來自 `fastcomments-react` 的每個小工具都以相同名稱提供：

| 元件 | Handle 型別 | 嵌入載入 |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | 旗艦即時留言小工具 |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | 內嵌留言數徽章 |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | 串流即時聊天小工具 |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | 文字錨定協作聊天 |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | 基於區域的圖片留言 |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | 最新留言動態 |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | 最新討論動態 |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | 星級評分摘要 |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | 最多留言頁面排行榜 |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | 個別使用者活動時間軸 |

### 附加到現有元素的小工具

`FastCommentsCollabChatWidget` 和 `FastCommentsImageChatWidget` 會掛載到呼叫者提供的元素上。傳入一個 `targetRef` 存取器，在元素掛載後回傳該元素：

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

傳入 `region="eu"` 以將小工具流量路由至 EU 叢集。