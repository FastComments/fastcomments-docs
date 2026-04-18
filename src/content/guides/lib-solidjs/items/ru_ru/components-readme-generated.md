---
Каждый виджет из `fastcomments-react` доступен под тем же именем:

| Компонент | Тип handle | Загружаемый виджет |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Флагманский виджет живых комментариев |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Инлайн-бейдж с количеством комментариев |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Потоковый виджет живого чата |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Совместный чат, привязанный к тексту |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Комментарии по регионам изображения |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Лента последних комментариев |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Лента последних обсуждений |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Сводка по звездному рейтингу |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Рейтинг страниц по количеству комментариев |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Хронология активности пользователя |

### Виджеты, которые монтируются в существующий элемент

`FastCommentsCollabChatWidget` и `FastCommentsImageChatWidget` монтируются в элемент, переданный вызывающим. Передайте accessor `targetRef`, который возвращает элемент после монтирования:

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

Укажите `region="eu"`, чтобы маршрутизировать трафик виджета через кластер ЕС.
---