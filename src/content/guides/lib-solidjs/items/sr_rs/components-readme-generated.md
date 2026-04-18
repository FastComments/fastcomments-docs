Сваки видџет из `fastcomments-react` доступан је под истим именом:

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Главни видџет за коментарисање уживо |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Инлајн значка броја коментара |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Видџет за стримовано ћаскање уживо |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Сараднички чат везан за текст |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Коментари на слици по регионима |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Фид недавних коментара |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Фид недавних дискусија |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Сажетак оцењивања звездицама |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Табела највише коментарисаних страница |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Трака активности по кориснику |

### Видџети који се прикачују на постојећи елемент

`FastCommentsCollabChatWidget` и `FastCommentsImageChatWidget` монтирају се у елемент који обезбеђује позивач. Проследите accessor `targetRef` који враћа елемент након што је монтиран:

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

### Региони

Проследите `region="eu"` да усмерите саобраћај видџета кроз ЕУ кластер.