| Komponente | Beschreibung |
| --- | --- |
| `FastComments` | Kommentierungs-Widget mit Antworten, Abstimmungen und mehr |
| `FastCommentsCommentCount` | Zeigt die Anzahl der Kommentare für eine Seite an |
| `FastCommentsImageChat` | Kommentare zur Bildannotation |
| `FastCommentsLiveChat` | Live-Chat-Widget |
| `FastCommentsCollabChat` | Kollaboratives Inline-Kommentieren |
| `FastCommentsReviewsSummary` | Zusammenfassung von Sternbewertungen |
| `FastCommentsUserActivityFeed` | Benutzer-Aktivitäts-Feed |

Alle Komponenten werden aus der Paketwurzel exportiert:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```