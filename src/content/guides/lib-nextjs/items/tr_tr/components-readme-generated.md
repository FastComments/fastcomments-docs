| Bileşen | Açıklama |
| --- | --- |
| `FastComments` | Yanıtlar, oy verme ve daha fazlasını içeren yorum bileşeni |
| `FastCommentsCommentCount` | Bir sayfanın yorum sayısını gösterir |
| `FastCommentsImageChat` | Görüntü üzerinde açıklama yorumları |
| `FastCommentsLiveChat` | Canlı sohbet bileşeni |
| `FastCommentsCollabChat` | İşbirlikçi satır içi yorumlar |
| `FastCommentsReviewsSummary` | Yıldız derecelendirmeli inceleme özeti |
| `FastCommentsUserActivityFeed` | Kullanıcı etkinlik akışı |

Tüm bileşenler paket kökünden dışa aktarılır:

```tsx
import {
    FastComments,
    FastCommentsLiveChat,
    FastCommentsReviewsSummary,
} from 'fastcomments-nextjs';
```