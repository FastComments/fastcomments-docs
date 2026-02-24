---
| Komponent | Beskrivelse |
| --- | --- |
| `FastComments` | Kommenteringswidget med svar, afstemning og mere |
| `FastCommentsCommentCount` | Viser antallet af kommentarer for en side |
| `FastCommentsImageChat` | Billedannoteringskommentarer |
| `FastCommentsLiveChat` | Live chat-widget |
| `FastCommentsCollabChat` | Samarbejdende inline-kommentering |
| `FastCommentsReviewsSummary` | Opsummering af stjernebedømmelser |
| `FastCommentsUserActivityFeed` | Brugeraktivitetsfeed |

Alle komponenter eksporteres fra pakkens rod:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---