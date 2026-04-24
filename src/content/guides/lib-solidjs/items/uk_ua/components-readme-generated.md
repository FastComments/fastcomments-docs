Кожен віджет з `fastcomments-react` доступний під тією ж назвою:

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Флагманський віджет для коментування в реальному часі |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Вбудований бейдж із кількістю коментарів |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Віджет потокового чату в реальному часі |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Спільний чат, прив'язаний до тексту |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Коментарі до зображень на основі регіонів |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Стрічка останніх коментарів |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Стрічка останніх обговорень |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Зведення рейтингу за зірками |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Рейтинг сторінок за кількістю коментарів |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Хронологія активності користувача |

### Віджети, які приєднуються до існуючого елемента

`FastCommentsCollabChatWidget` та `FastCommentsImageChatWidget` монтуються в елемент, що надається викликачем. Передайте аксесор `targetRef`, який повертає елемент після монтування:

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

### Регіони

Передайте `region="eu"`, щоб направити трафік віджетів через кластер ЄС.