---
| Komponenta | Opis |
| --- | --- |
| `FastComments` | Vtičnik za komentarje z odgovori, glasovanjem in drugimi funkcijami |
| `FastCommentsCommentCount` | Prikaže število komentarjev za stran |
| `FastCommentsImageChat` | Komentarji za označevanje slik |
| `FastCommentsLiveChat` | Vtičnik za klepet v živo |
| `FastCommentsCollabChat` | Sodelovalno vgrajeno komentiranje |
| `FastCommentsReviewsSummary` | Povzetek ocen z zvezdicami |
| `FastCommentsUserActivityFeed` | Vir aktivnosti uporabnika |

Vse komponente so izvožene iz korena paketa:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---