---
| Component | Description |
| --- | --- |
| `FastComments` | Виджет комментариев с ответами, голосованием и другими возможностями |
| `FastCommentsCommentCount` | Отображает количество комментариев на странице |
| `FastCommentsImageChat` | Аннотации к изображениям (комментарии) |
| `FastCommentsLiveChat` | Виджет чата в реальном времени |
| `FastCommentsCollabChat` | Совместное встроенное комментирование |
| `FastCommentsReviewsSummary` | Сводка отзывов со звёздным рейтингом |
| `FastCommentsUserActivityFeed` | Лента активности пользователя |

Все компоненты экспортируются из корня пакета:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---