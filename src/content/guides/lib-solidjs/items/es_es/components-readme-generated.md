Cada widget de `fastcomments-react` está disponible con el mismo nombre:

| Componente | Tipo de handle | Incrustación cargada |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Widget principal de comentarios en vivo |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Insignia de recuento de comentarios en línea |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Widget de chat en vivo en streaming |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Chat colaborativo anclado a texto |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Comentarios en imágenes basados en regiones |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Feed de comentarios recientes |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Feed de discusiones recientes |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Resumen de valoraciones con estrellas |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Clasificación de páginas más comentadas |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Cronología de actividad por usuario |

### Widgets que se adjuntan a un elemento existente

`FastCommentsCollabChatWidget` y `FastCommentsImageChatWidget` se montan en un elemento proporcionado por el llamador. Pasa un accesor `targetRef` que devuelve el elemento una vez montado:

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

### Regiones

Pasa `region="eu"` para enrutar el tráfico del widget a través del clúster de la UE.