---
| Компонента | Опис |
| --- | --- |
| `FastComments` | Видгет за коментаре са одговорима, гласањем и још више |
| `FastCommentsCommentCount` | Приказује број коментара за страницу |
| `FastCommentsImageChat` | Коментари за анотирање слика |
| `FastCommentsLiveChat` | Видгет за ћаскање уживо |
| `FastCommentsCollabChat` | Колаборативно инлајн коментарисање |
| `FastCommentsReviewsSummary` | Резиме рецензија са оценама у звездицама |
| `FastCommentsUserActivityFeed` | Ток активности корисника |

Све компоненте се извозе из корена пакета:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---