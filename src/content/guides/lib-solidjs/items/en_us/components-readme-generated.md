Every widget from `fastcomments-react` is available under the same name:

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Flagship live commenting widget |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Inline comment-count badge |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Streaming live-chat widget |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Text-anchored collaborative chat |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Region-based image comments |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Recent-comments feed |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Recent-discussions feed |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Star-rating summary |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Top-commented pages leaderboard |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Per-user activity timeline |

### Widgets that attach to an existing element

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

### Regions

Pass `region="eu"` to route widget traffic through the EU cluster.