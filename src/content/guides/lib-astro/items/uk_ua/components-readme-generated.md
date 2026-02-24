---
| Компонент | Опис |
| --- | --- |
| `FastComments` | Віджет коментарів з відповідями, голосуванням та іншими можливостями |
| `FastCommentsCommentCount` | Відображає кількість коментарів для сторінки |
| `FastCommentsImageChat` | Коментарі з анотаціями на зображеннях |
| `FastCommentsLiveChat` | Віджет чату в реальному часі |
| `FastCommentsCollabChat` | Спільне вбудоване коментування |
| `FastCommentsReviewsSummary` | Зведення оглядів із рейтингом у вигляді зірок |
| `FastCommentsUserActivityFeed` | Стрічка активності користувача |

Усі компоненти експортуються з кореня пакета:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---