Каждый виджет из `fastcomments-react` доступен под тем же именем:

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Флагманский виджет для живого комментирования |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Встроенный бейдж с подсчётом комментариев |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Потоковый виджет живого чата |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Совместный чат, привязанный к тексту |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Комментирование регионов изображения |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Лента последних комментариев |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Лента последних обсуждений |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Сводка рейтинга в звёздах |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Рейтинг страниц с наибольшим количеством комментариев |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Хронология активности пользователя |

### Виджеты, которые монтируются в существующий элемент

`FastCommentsCollabChatWidget` и `FastCommentsImageChatWidget` монтируются в элемент, предоставленный вызывающей стороной.
Передайте accessor `targetRef`, который возвращает элемент после его монтирования:

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

### Регионы

Передайте `region="eu"`, чтобы направить трафик виджета через кластер ЕС.
---