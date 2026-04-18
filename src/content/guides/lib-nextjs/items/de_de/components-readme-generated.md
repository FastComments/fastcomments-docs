| Component | Description |
| --- | --- |
| `FastComments` | Kommentar-Widget mit Antworten, Abstimmungen und mehr |
| `FastCommentsCommentCount` | Zeigt die Anzahl der Kommentare für eine Seite an |
| `FastCommentsImageChat` | Kommentare zur Bildannotation |
| `FastCommentsLiveChat` | Live-Chat-Widget |
| `FastCommentsCollabChat` | Kollaboratives Inline-Kommentieren |
| `FastCommentsReviewsSummary` | Zusammenfassung der Sternbewertungen |
| `FastCommentsUserActivityFeed` | Benutzer-Aktivitätsfeed |

Alle Komponenten werden aus dem Paketstamm exportiert:

```tsx
import {
    FastComments,
    FastCommentsLiveChat,
    FastCommentsReviewsSummary,
} from 'fastcomments-nextjs';
```