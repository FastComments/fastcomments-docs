---
Cada widget do `fastcomments-react` está disponível com o mesmo nome:

| Componente | Tipo de handle | Embed carregado |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Widget principal de comentários ao vivo |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Badge de contagem de comentários em linha |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Widget de chat ao vivo em streaming |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Chat colaborativo ancorado ao texto |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Comentários por região na imagem |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Feed de comentários recentes |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Feed de discussões recentes |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Resumo de avaliação por estrelas |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | Classificação das páginas mais comentadas |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Linha do tempo de atividade por usuário |

### Widgets que se anexam a um elemento existente

`FastCommentsCollabChatWidget` and `FastCommentsImageChatWidget` mount into a caller-supplied
element. Passe um acessor `targetRef` que retorne o elemento depois que ele for montado:

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

### Regiões

Passe `region="eu"` para rotear o tráfego do widget através do cluster da UE.
---