---
| Component | 説明 |
| --- | --- |
| `FastComments` | 返信、投票などの機能を備えたコメントウィジェット |
| `FastCommentsCommentCount` | ページのコメント数を表示 |
| `FastCommentsImageChat` | 画像注釈コメント |
| `FastCommentsLiveChat` | ライブチャットウィジェット |
| `FastCommentsCollabChat` | 共同インラインコメント |
| `FastCommentsReviewsSummary` | 星評価レビューの概要 |
| `FastCommentsUserActivityFeed` | ユーザーアクティビティフィード |

すべてのコンポーネントはパッケージのルートからエクスポートされています:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---