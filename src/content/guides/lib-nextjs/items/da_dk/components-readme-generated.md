| Komponent | Beskrivelse |
| --- | --- |
| `FastComments` | Kommentar-widget med svar, stemmefunktion og mere |
| `FastCommentsCommentCount` | Viser antal kommentarer for en side |
| `FastCommentsImageChat` | Billedannoteringskommentarer |
| `FastCommentsLiveChat` | Live chat-widget |
| `FastCommentsCollabChat` | Samarbejdende inline-kommentering |
| `FastCommentsReviewsSummary` | Sammendrag af anmeldelser med stjernebedømmelse |
| `FastCommentsUserActivityFeed` | Brugeraktivitet-feed |

Alle komponenter eksporteres fra pakkens rod:

```tsx
import {
    FastComments,
    FastCommentsLiveChat,
    FastCommentsReviewsSummary,
} from 'fastcomments-nextjs';
```