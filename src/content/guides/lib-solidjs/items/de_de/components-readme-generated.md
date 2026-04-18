---
Jedes Widget aus `fastcomments-react` ist unter demselben Namen verfügbar:

| Komponente | Handle-Typ | Geladene Einbettung |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Haupt-Live-Kommentar-Widget |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Inline-Kommentarzähler-Badge |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Streaming-Live-Chat-Widget |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Textverankerter kollaborativer Chat |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Regionsbasierte Bildkommentare |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Feed mit aktuellen Kommentaren |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Feed mit aktuellen Diskussionen |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Zusammenfassung der Sternebewertungen |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Rangliste der am häufigsten kommentierten Seiten |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Aktivitätsverlauf pro Nutzer |

### Widgets, die an ein vorhandenes Element angehängt werden

`FastCommentsCollabChatWidget` und `FastCommentsImageChatWidget` werden in ein vom Aufrufer bereitgestelltes
Element eingebunden. Übergeben Sie einen `targetRef`-Accessor, der das Element zurückgibt, sobald es montiert ist:

```tsx
import { FastCommentsImageChatWidget } from 'fastcomments-solidjs';

export default function ImageChat() {
  let img: HTMLImageElement | undefined;
  return (
    <>
      <img ref={img} src="/screenshot.png" alt="" />
      <FastCommentsImageChatWidget
        tenantId="demo"
        urlId="my-image"
        targetRef={() => img}
      />
    </>
  );
}
```

### Regionen

Geben Sie `region="eu"` an, um den Widget-Verkehr über den EU-Cluster zu leiten.
---