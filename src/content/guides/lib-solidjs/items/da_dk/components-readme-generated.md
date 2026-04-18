---
Hver widget fra `fastcomments-react` er tilgængelig under samme navn:

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Flagship live-kommenterings-widget |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Inline badge med kommentarantal |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Streaming live-chat-widget |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Tekst-ankret samarbejds-chat |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Regionsbaserede billedkommentarer |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Feed med nylige kommentarer |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Feed med nylige diskussioner |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Opsummering af stjernebedømmelser |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Top-liste over mest kommenterede sider |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Per-bruger aktivitetstidslinje |

### Widgets der tilknyttes et eksisterende element

`FastCommentsCollabChatWidget` og `FastCommentsImageChatWidget` monteres i et element leveret af kaldende kode. Pass en `targetRef` accessor som returnerer elementet, når det er monteret:

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

### Regioner

Angiv `region="eu"` for at rute widget-trafik gennem EU-klyngen.
---