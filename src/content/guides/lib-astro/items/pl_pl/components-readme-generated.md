| Component | Description |
| --- | --- |
| `FastComments` | Widżet komentarzy z odpowiedziami, głosowaniem i innymi funkcjami |
| `FastCommentsCommentCount` | Wyświetla liczbę komentarzy dla strony |
| `FastCommentsImageChat` | Komentarze z adnotacjami obrazów |
| `FastCommentsLiveChat` | Widżet czatu na żywo |
| `FastCommentsCollabChat` | Współpraca przy komentarzach osadzonych w tekście |
| `FastCommentsReviewsSummary` | Podsumowanie recenzji z ocenami w gwiazdkach |
| `FastCommentsUserActivityFeed` | Kanał aktywności użytkownika |

Wszystkie komponenty są eksportowane z głównego katalogu pakietu:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---