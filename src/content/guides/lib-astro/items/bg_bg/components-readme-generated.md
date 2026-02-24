| Компонент | Описание |
| --- | --- |
| `FastComments` | Уиджет за коментари с отговори, гласуване и още |
| `FastCommentsCommentCount` | Показва броя коментари за страница |
| `FastCommentsImageChat` | Коментари за анотации на изображения |
| `FastCommentsLiveChat` | Уиджет за чат на живо |
| `FastCommentsCollabChat` | Съвместни вградени коментари |
| `FastCommentsReviewsSummary` | Резюме на ревюта със звездни оценки |
| `FastCommentsUserActivityFeed` | Лента с активност на потребителя |

Всички компоненти са експортирани от корена на пакета:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```