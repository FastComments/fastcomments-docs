Chaque widget de `fastcomments-react` est disponible sous le même nom :

| Component | Handle type | Embed chargé |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Widget phare de commentaires en direct |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Badge intégré du compteur de commentaires |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Widget de chat en direct en streaming |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Chat collaboratif ancré au texte |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Commentaires d'image basés sur des régions |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Flux des commentaires récents |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Flux des discussions récentes |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Résumé des évaluations par étoiles |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Classement des pages les plus commentées |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Fil d'activité par utilisateur |

### Widgets qui s'attachent à un élément existant

`FastCommentsCollabChatWidget` et `FastCommentsImageChatWidget` s'installent dans un élément fourni par l'appelant. Passez un accesseur `targetRef` qui renvoie l'élément une fois monté :

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

Passez `region="eu"` pour diriger le trafic du widget via le cluster de l'UE.