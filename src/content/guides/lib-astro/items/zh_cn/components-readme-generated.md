| Component | Description |
| --- | --- |
| `FastComments` | 带回复、投票及更多功能的评论小部件 |
| `FastCommentsCommentCount` | 显示页面的评论数量 |
| `FastCommentsImageChat` | 图像注释评论 |
| `FastCommentsLiveChat` | 实时聊天小部件 |
| `FastCommentsCollabChat` | 协作式内联评论 |
| `FastCommentsReviewsSummary` | 星级评分评论摘要 |
| `FastCommentsUserActivityFeed` | 用户活动提要 |

所有组件都从包根导出：

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```