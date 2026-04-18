Każdy widget z `fastcomments-react` jest dostępny pod tą samą nazwą:

| Komponent | Typ uchwytu | Ładowany embed |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Flagowy widget komentarzy na żywo |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Wbudowana plakietka z liczbą komentarzy |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Strumieniowy widget czatu na żywo |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Współpracujący czat powiązany z tekstem |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Komentarze obrazów oparte na regionach |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Kanał najnowszych komentarzy |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Kanał najnowszych dyskusji |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Podsumowanie ocen gwiazdkowych |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Ranking stron z największą liczbą komentarzy |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Oś czasu aktywności użytkownika |

### Widgety, które montują się w istniejącym elemencie

`FastCommentsCollabChatWidget` i `FastCommentsImageChatWidget` montują się w elemencie dostarczonym przez wywołującego. Przekaż funkcję `targetRef`, która zwraca element po zamontowaniu:

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

### Regiony

Przekaż `region="eu"`, aby kierować ruch widgetu przez klaster UE.