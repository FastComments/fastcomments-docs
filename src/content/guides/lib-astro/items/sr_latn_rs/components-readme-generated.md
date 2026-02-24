---
| Komponenta | Opis |
| --- | --- |
| `FastComments` | Vidžet za komentare sa odgovorima, glasovima i još mnogo toga |
| `FastCommentsCommentCount` | Prikazuje broj komentara na stranici |
| `FastCommentsImageChat` | Komentari za anotacije slika |
| `FastCommentsLiveChat` | Vidžet za razgovor uživo |
| `FastCommentsCollabChat` | Kolaborativno inline komentarisanje |
| `FastCommentsReviewsSummary` | Sažetak ocena pomoću zvezdica |
| `FastCommentsUserActivityFeed` | Tok aktivnosti korisnika |

Sve komponente se izvoze iz korena paketa:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---