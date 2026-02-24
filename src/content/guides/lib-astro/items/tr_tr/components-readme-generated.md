---
| Bileşen | Açıklama |
| --- | --- |
| `FastComments` | Yanıtlar, oy verme ve daha fazlasıyla yorum bileşeni |
| `FastCommentsCommentCount` | Bir sayfanın yorum sayısını gösterir |
| `FastCommentsImageChat` | Görsel açıklama yorumları |
| `FastCommentsLiveChat` | Canlı sohbet bileşeni |
| `FastCommentsCollabChat` | İşbirlikçi satır içi yorumlama |
| `FastCommentsReviewsSummary` | Yıldız derecelendirme incelemeleri özeti |
| `FastCommentsUserActivityFeed` | Kullanıcı etkinlik akışı |

Tüm bileşenler paket kökünden dışa aktarılır:

```astro
---
import { FastComments, FastCommentsLiveChat } from 'fastcomments-astro';
---
```
---