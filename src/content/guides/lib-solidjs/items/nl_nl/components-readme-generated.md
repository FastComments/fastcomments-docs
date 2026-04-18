Elk widget uit `fastcomments-react` is beschikbaar onder dezelfde naam:

| Component | Handle type | Ingesloten embed |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Hoofd-widget voor livecommentaar |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Inline-badge met aantal reacties |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Streaming livechat-widget |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Samenwerkende chat verankerd aan tekst |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Regiogebaseerde afbeeldingsreacties |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Feed met recente reacties |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Feed met recente discussies |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Samenvatting van sterrenbeoordelingen |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Ranglijst van pagina's met meeste reacties |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Activiteitstijdlijn per gebruiker |

### Widgets die aan een bestaand element worden gekoppeld

`FastCommentsCollabChatWidget` en `FastCommentsImageChatWidget` worden gemount in een door de aanroeper aangeleverd element. Geef een `targetRef`-toegangsfunctie door die het element teruggeeft zodra het is gemonteerd:

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

### Regio's

Geef `region="eu"` op om het widgetverkeer via het EU-cluster te routeren.