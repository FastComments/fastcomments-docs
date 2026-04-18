---
`fastcomments-react` のすべてのウィジェットは同じ名前で利用できます：

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | フラグシップのライブコメントウィジェット |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | インラインのコメント数バッジ |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | ストリーミングのライブチャットウィジェット |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | テキストに紐づく共同チャット |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | 領域ベースの画像コメント |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | 最近のコメントフィード |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | 最近のディスカッションフィード |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | 星評価の概要 |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | コメント数上位ページのランキング |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | ユーザーごとのアクティビティタイムライン |

### Widgets that attach to an existing element

`FastCommentsCollabChatWidget` と `FastCommentsImageChatWidget` は、呼び出し元が提供する要素にマウントされます。マウント後にその要素を返す `targetRef` アクセサを渡してください：

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

### リージョン

ウィジェットのトラフィックをEUクラスター経由にするには `region="eu"` を渡します。
---