Svaki widget iz `fastcomments-react` dostupan je pod istim nazivom:

| Komponenta | Tip handlea | Ugrađeni widget |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Glavni widget za komentare u stvarnom vremenu |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Inline oznaka broja komentara |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Widget za live chat u stvarnom vremenu |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Kolaborativni chat vezan uz tekst |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Komentari na sliku po regijama |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Feed nedavnih komentara |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Feed nedavnih rasprava |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Sažetak ocjena zvjezdicama |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Ljestvica stranica s najviše komentara |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Vremenska linija aktivnosti po korisniku |

### Widgeti koji se pričvršćuju na postojeći element

`FastCommentsCollabChatWidget` i `FastCommentsImageChatWidget` montiraju se u element koji
dodijeli pozivatelj. Proslijedite `targetRef` pristupnik koji vraća element nakon montiranja:

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

Proslijedite `region="eu"` za usmjeravanje prometa widgeta kroz EU klaster.