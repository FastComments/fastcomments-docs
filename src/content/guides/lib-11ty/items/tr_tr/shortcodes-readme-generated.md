| Kısa Kod | Açıklama |
| --- | --- |
| `fastcomments` | Yanıtlar, oy verme ve daha fazlasını içeren yorum bileşeni |
| `fastcommentsCommentCount` | Bir sayfanın yorum sayısını gösterir |
| `fastcommentsImageChat` | Görüntü üzerine açıklama yorumları |
| `fastcommentsLiveChat` | Canlı sohbet bileşeni |
| `fastcommentsCollabChat` | İşbirlikçi satır içi yorumlar |
| `fastcommentsRecentComments` | Sitedeki son yorumlar |
| `fastcommentsRecentDiscussions` | Son etkin tartışma başlıkları |
| `fastcommentsReviewsSummary` | Yıldız derecelendirmeli incelemeler özeti |
| `fastcommentsTopPages` | En çok tartışılan sayfalar |
| `fastcommentsUserActivityFeed` | Kullanıcı etkinlik akışı |

### Örnekler

```njk
{# Metin içinde yorum sayısı #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Canlı sohbet #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# İşbirlikçi sohbet — bir içerik öğesini CSS seçiciyle hedefleyin #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Görüntü sohbeti — bir resim öğesini CSS seçiciyle hedefleyin #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# İncelemeler özeti #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Kullanıcı etkinlik akışı #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```