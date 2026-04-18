Svaki widget iz `fastcomments-react` dostupan je pod istim imenom:

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Glavni widget za komentare uživo |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Inline bedž s brojačem komentara |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Streaming widget za chat uživo |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Kolaborativni chat vezan za tekst |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Komentari na slici temeljeni na regijama |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Feed nedavnih komentara |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Feed nedavnih rasprava |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Sažetak ocjena zvjezdicama |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Ljestvica najkomentiranijih stranica |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Vremenska linija aktivnosti korisnika |

### Widgets that attach to an existing element

`FastCommentsCollabChatWidget` i `FastCommentsImageChatWidget` se montiraju u element koji pozivatelj osigura. Proslijedite accessor `targetRef` koji vraća element nakon što je montiran:

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

### Regije

Proslijedite `region="eu"` kako biste usmjerili promet widgeta kroz EU klaster.