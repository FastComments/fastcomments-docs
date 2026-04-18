Chaque widget de `fastcomments-react` est disponible sous le même nom :

| Composant | Type de handle | Intégration chargée |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Widget principal de commentaires en direct |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Badge intégré du nombre de commentaires |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Widget de clavardage en direct en continu |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Chat collaboratif ancré au texte |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Commentaires d'image par région |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Fil des commentaires récents |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Fil des discussions récentes |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Résumé des évaluations par étoiles |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Palmarès des pages les plus commentées |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Chronologie d'activité par utilisateur |

### Widgets qui s'attachent à un élément existant

`FastCommentsCollabChatWidget` et `FastCommentsImageChatWidget` se montent dans un élément fourni par l'appelant. Fournissez un accesseur `targetRef` qui retourne l'élément une fois monté :

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

### Régions

Passez `region="eu"` pour acheminer le trafic du widget via le cluster de l'UE.