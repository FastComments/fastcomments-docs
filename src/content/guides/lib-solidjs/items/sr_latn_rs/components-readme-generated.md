Svaki widget iz `fastcomments-react` dostupan je pod istim imenom:

| Komponenta | Tip handle-a | Učitani embed |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Vodeći widget za komentarisanje uživo |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Inline značka sa brojačem komentara |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Widget za streaming chat uživo |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Kolaborativni chat vezan za tekst |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Komentari na slici po regionima |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Feed najnovijih komentara |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Feed najnovijih diskusija |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Rezime ocena zvezdicama |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Lista najkomentaranijih stranica |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Vremenska linija aktivnosti po korisniku |

### Widgeti koji se prikače na postojeći element

`FastCommentsCollabChatWidget` and `FastCommentsImageChatWidget` montiraju se u element koji pozivalac obezbedi.
Prosledite accessor `targetRef` koji vraća element nakon što je montiran:

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

Prosledite `region="eu"` da usmerite saobraćaj widgeta kroz EU klaster.