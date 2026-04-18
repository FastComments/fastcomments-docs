---
Всеки widget от `fastcomments-react` е наличен под същото име:

| Component | Handle type | Вградено съдържание |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Флагмански widget за коментари в реално време |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Инлайн значка за брой коментари |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Поточен widget за чат в реално време |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Съвместен чат, прикрепен към текст |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Коментари върху изображения по региони |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Фийд с последни коментари |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Фийд с последни дискусии |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Обобщение на оценки със звезди |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Класация на най-коментираните страници |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Хронология на активността на потребителя |

### Уиджети, които се прикрепят към съществуващ елемент

`FastCommentsCollabChatWidget` и `FastCommentsImageChatWidget` се монтират в елемент, предоставен от извикващия. Подайте accessor `targetRef`, който връща елемента след монтиране:

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

Задайте `region="eu"` за да пренасочите трафика на widget-а през клъстъра в ЕС.
---