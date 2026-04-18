Her `fastcomments-react` içindeki her widget aynı isimle kullanılabilir:

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | Amiral gemisi canlı yorum widget'ı |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | Satır içi yorum sayısı rozeti |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | Akışlı canlı sohbet widget'ı |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | Metne dayalı işbirlikçi sohbet |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | Bölge tabanlı resim yorumları |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | Son yorumlar beslemesi |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | Son tartışmalar beslemesi |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | Yıldız değerlendirme özeti |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | En çok yorumlanan sayfalar lider tablosu |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | Kullanıcı bazlı etkinlik zaman çizelgesi |

### Mevcut bir öğeye eklenen widget'lar

`FastCommentsCollabChatWidget` ve `FastCommentsImageChatWidget`, çağıran tarafından sağlanan bir
öğeye monte olur. Monte edildikten sonra öğeyi döndüren bir `targetRef` erişici sağlayın:

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

### Bölgeler

Widget trafiğini AB kümesi üzerinden yönlendirmek için `region="eu"` verin.