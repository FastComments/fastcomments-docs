Svaki widget iz `fastcomments-react` dostupan je pod istim imenom:

| Komponenta | Tip handle-a | Ugrađeni sadržaj |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Glavni widget za komentarisanje uživo |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Ugrađena oznaka broja komentara |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Widget za streaming chat uživo |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Kolaborativni chat vezan za tekst |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Komentari na slici vezani za regije |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Tok nedavnih komentara |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Tok nedavnih diskusija |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Sažetak ocjena sa zvjezdicama |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Lista najkomentisanijih stranica |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Vremenska linija aktivnosti po korisniku |

### Widgeti koji se montiraju u postojeći element

`FastCommentsCollabChatWidget` i `FastCommentsImageChatWidget` se montiraju u element koji obezbijedi pozivalac. Proslijedite accessor `targetRef` koji vraća element nakon što je montiran:

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

Proslijedite `region="eu"` da usmjerite saobraćaj widgeta kroz EU klaster.