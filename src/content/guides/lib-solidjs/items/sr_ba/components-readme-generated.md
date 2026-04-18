---
Svaki widget iz `fastcomments-react` dostupan je pod istim nazivom:

| Komponenta | Tip handle-a | Učitani embed |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Glavni widget za komentarisanje uživo |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Inline bedž sa brojem komentara |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Streaming widget za chat uživo |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Kolaborativni chat prikačen uz tekst |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Komentari na slici po regijama |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Feed najnovijih komentara |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Feed najnovijih diskusija |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Sažetak ocjena u zvjezdicama |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Lista najkomentiranijih stranica |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Vremenska linija aktivnosti po korisniku |

### Widgeti koji se montiraju u postojeći element

`FastCommentsCollabChatWidget` i `FastCommentsImageChatWidget` montiraju se u element koji obezbijedi pozivalac.
Prosledite `targetRef` accessor koji vraća element nakon što je montiran:

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

Prosledite `region="eu"` da usmjerite saobraćaj widgeta kroz EU klaster.
---