| Komponenta | Opis |
| --- | --- |
| `FastComments` | Widget za komentiranje s odgovorima, glasovanjem i ostalim značajkama |
| `FastCommentsCommentCount` | Prikazuje broj komentara za stranicu |
| `FastCommentsImageChat` | Komentari za označavanje slika |
| `FastCommentsLiveChat` | Widget za chat uživo |
| `FastCommentsCollabChat` | Suradničko inline komentiranje |
| `FastCommentsReviewsSummary` | Sažetak recenzija s ocjenama u zvjezdicama |
| `FastCommentsUserActivityFeed` | Feed aktivnosti korisnika |

Sve komponente se izvoze iz korijena paketa:

```tsx
import {
    FastComments,
    FastCommentsLiveChat,
    FastCommentsReviewsSummary,
} from 'fastcomments-nextjs';
```