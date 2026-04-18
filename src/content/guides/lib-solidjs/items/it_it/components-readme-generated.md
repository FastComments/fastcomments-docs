Ogni widget di `fastcomments-react` è disponibile con lo stesso nome:

| Componente | Tipo di handle | Embed caricato |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Widget principale per commenti in tempo reale |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Badge inline per il conteggio dei commenti |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Widget di live chat in streaming |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Chat collaborativa ancorata al testo |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Commenti su immagine basati su regioni |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Feed dei commenti recenti |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Feed delle discussioni recenti |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Riepilogo valutazioni a stelle |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Classifica delle pagine più commentate |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Cronologia attività per utente |

### Widget che si agganciano a un elemento esistente

`FastCommentsCollabChatWidget` e `FastCommentsImageChatWidget` si montano in un elemento fornito dal chiamante.
Passa un accessor `targetRef` che restituisce l'elemento una volta montato:

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

### Regioni

Passa `region="eu"` per instradare il traffico del widget attraverso il cluster UE.
---