`fastcomments-react` からのすべてのウィジェットは、同じ名前で利用可能です:

| コンポーネント | ハンドル型 | 埋め込みの説明 |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | フラッグシップのライブコメントウィジェット |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | インラインコメント数バッジ |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | ストリーミングライブチャットウィジェット |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | テキストアンカー型コラボレーションチャット |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | 領域ベースの画像コメント |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | 最近のコメントフィード |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | 最近のディスカッションフィード |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | 星評価のサマリー |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | コメント数の多いページのランキング |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | ユーザーごとのアクティビティタイムライン |

### 既存の要素にアタッチするウィジェット

`FastCommentsCollabChatWidget` と `FastCommentsImageChatWidget` は、呼び出し側が指定した要素にマウントされます。マウント後に要素を返す `targetRef` アクセサーを渡してください:

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

`region="eu"` を渡すと、ウィジェットのトラフィックが EU クラスター経由でルーティングされます。