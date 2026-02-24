---
| Компонент | Описание |
| --- | --- |
| `FastComments` | Виджет комментариев с ответами, голосованием и прочими функциями |
| `FastCommentsCommentCount` | Отображает количество комментариев на странице |
| `FastCommentsImageChat` | Комментарии с аннотациями изображений |
| `FastCommentsLiveChat` | Виджет живого чата |
| `FastCommentsCollabChat` | Совместное встроенное комментирование |
| `FastCommentsReviewsSummary` | Сводка отзывов со звездным рейтингом |
| `FastCommentsUserActivityFeed` | Лента активности пользователя |

Все компоненты экспортируются из корня пакета:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---