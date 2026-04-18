---
Vsak pripomoček iz `fastcomments-react` je na voljo pod istim imenom:

| Komponenta | Tip ročaja | Naložena vdelava |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Glavni pripomoček za komentiranje v živo |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Vrstični števec komentarjev |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Pretakajoči pripomoček za klepet v živo |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Sodelovalni klepet, pritrjen na besedilo |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Komentarji na sliko po regijah |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Tok nedavnih komentarjev |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Tok nedavnih razprav |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Povzetek ocen z zvezdicami |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Lestvica najbolj komentiranih strani |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Časovnica aktivnosti po uporabniku |

### Pripomočki, ki se pritnejo na obstoječi element

`FastCommentsCollabChatWidget` and `FastCommentsImageChatWidget` mount into a caller-supplied
element. Posredujte dostopnik `targetRef`, ki vrne element po namestitvi:

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

Posredujte `region="eu"` za usmerjanje prometa vdelav prek EU grozda.
---