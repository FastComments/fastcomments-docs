---
| Компонента | Опис |
| --- | --- |
| `FastComments` | Видгет за коментаре са одговорима, гласањем и још много тога |
| `FastCommentsCommentCount` | Приказује број коментара за страницу |
| `FastCommentsImageChat` | Коментари за анотирање слика |
| `FastCommentsLiveChat` | Видгет за ћаскање уживо |
| `FastCommentsCollabChat` | Сарадничко инлајн коментарисање |
| `FastCommentsReviewsSummary` | Сажетак рецензија са оцјенама у звјездицама |
| `FastCommentsUserActivityFeed` | Фид активности корисника |

Све компоненте се извозе из коријена пакета:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---