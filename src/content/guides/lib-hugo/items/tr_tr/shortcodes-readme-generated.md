---
| Shortcode | Açıklama |
| --- | --- |
| `fastcomments` | Yanıtlar, oy verme ve @bahsetmeler ile dallanmış yorumlar |
| `fastcomments-comment-count` | Tek bir sayfa için yorum sayısı |
| `fastcomments-comment-count-bulk` | Bir istekte birçok sayfa için yorum sayıları (bkz. [Toplu yorum sayıları](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Canlı sohbet bileşeni |
| `fastcomments-collab-chat` | Satır içi işbirlikçi yorumlama (gerektirir `target`) |
| `fastcomments-image-chat` | Görüntü açıklama yorumları (gerektirir `target`) |
| `fastcomments-recent-comments` | Site genelindeki son yorumlar |
| `fastcomments-recent-discussions` | Son zamanlarda etkin tartışma dizileri |
| `fastcomments-reviews-summary` | Yıldız derecelendirmeli incelemelerin özeti |
| `fastcomments-top-pages` | En çok tartışılan sayfalar |
| `fastcomments-user-activity-feed` | Kullanıcı başına etkinlik akışı (gerektirir `userId`) |

### Örnekler

Metin içinde yorum sayısı:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Canlı sohbet:

```text
\{{< fastcomments-live-chat >}}
```

İşbirlikçi sohbet, bir içerik öğesini CSS seçicisiyle hedefleme:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Görsel sohbet, bir görsel öğesini CSS seçicisiyle hedefleme:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

İncelemeler özeti:

```text
\{{< fastcomments-reviews-summary >}}
```

Kullanıcı etkinlik akışı:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```
---