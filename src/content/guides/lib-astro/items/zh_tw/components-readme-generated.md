| 元件 | 說明 |
| --- | --- |
| `FastComments` | 具有回覆、投票等功能的留言小工具 |
| `FastCommentsCommentCount` | 顯示頁面的留言數量 |
| `FastCommentsImageChat` | 圖片註解留言 |
| `FastCommentsLiveChat` | 即時聊天小工具 |
| `FastCommentsCollabChat` | 協作式內嵌留言 |
| `FastCommentsReviewsSummary` | 星級評分評論摘要 |
| `FastCommentsUserActivityFeed` | 使用者活動動態 |

所有元件皆從套件根目錄匯出：

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```