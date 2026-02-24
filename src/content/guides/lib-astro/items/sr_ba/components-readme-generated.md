---
| Компонента | Опис |
| --- | --- |
| `FastComments` | Видгет за коментаре са одговорима, гласањем и другим функцијама |
| `FastCommentsCommentCount` | Приказује број коментара за страницу |
| `FastCommentsImageChat` | Коментари за анотирање слика |
| `FastCommentsLiveChat` | Видгет за уживо ћаскање |
| `FastCommentsCollabChat` | Сарадничко инлајн коментарисање |
| `FastCommentsReviewsSummary` | Сажетак рецензија са оцењивањем у звездицама |
| `FastCommentsUserActivityFeed` | Фид активности корисника |

Све компоненте се экспортују из корена пакета:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---