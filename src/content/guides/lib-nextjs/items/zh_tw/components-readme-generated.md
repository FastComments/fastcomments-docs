| 元件 | 說明 |
| --- | --- |
| `FastComments` | 具備回覆、投票等功能的留言元件 |
| `FastCommentsCommentCount` | 顯示頁面留言數量 |
| `FastCommentsImageChat` | 圖片註解留言 |
| `FastCommentsLiveChat` | 即時聊天元件 |
| `FastCommentsCollabChat` | 協作式內嵌留言 |
| `FastCommentsReviewsSummary` | 星級評分評論摘要 |
| `FastCommentsUserActivityFeed` | 使用者活動動態 |

所有元件皆從套件根目錄匯出：

```tsx
import {
    FastComments,
    FastCommentsLiveChat,
    FastCommentsReviewsSummary,
} from 'fastcomments-nextjs';
```