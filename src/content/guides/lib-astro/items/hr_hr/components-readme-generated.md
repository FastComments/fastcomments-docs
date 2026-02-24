---
| Komponenta | Opis |
| --- | --- |
| `FastComments` | Widget za komentiranje s odgovorima, glasovanjem i više |
| `FastCommentsCommentCount` | Prikazuje broj komentara za stranicu |
| `FastCommentsImageChat` | Komentari za anotacije slika |
| `FastCommentsLiveChat` | Widget za chat uživo |
| `FastCommentsCollabChat` | Suradničko inline komentiranje |
| `FastCommentsReviewsSummary` | Sažetak recenzija sa zvjezdanim ocjenama |
| `FastCommentsUserActivityFeed` | Feed aktivnosti korisnika |

Sve komponente se izvoze iz korijena paketa:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---