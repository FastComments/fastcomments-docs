| Tag | Description |
| --- | --- |
| `fastcomments` | Yanıtlar, oylama, moderasyon ve gerçek zamanlı güncellemeler ile canlı yorum yapma |
| `fastcomments_comment_count` | Geçerli sayfa için yorum sayısı |
| `fastcomments_comment_count_bulk` | Bir liste/indeks sayfasındaki birçok sayfa için yorum sayıları |
| `fastcomments_live_chat` | Gerçek zamanlı akışlı sohbet bileşeni |
| `fastcomments_collab_chat` | İşbirlikçi satır içi yorumlar (metin açıklamaları) |
| `fastcomments_image_chat` | Görüntü açıklama yorumları |
| `fastcomments_recent_comments` | Site genelindeki son yorumlar |
| `fastcomments_recent_discussions` | Son etkin tartışma başlıkları |
| `fastcomments_reviews_summary` | Yıldızlı değerlendirmeler özeti |
| `fastcomments_top_pages` | En çok tartışılan sayfalar |
| `fastcomments_user_activity_feed` | Kullanıcı başına etkinlik akışı |

### Örnekler

```liquid
{% raw %}{# Yorum sayısı. Bileşen kendi etiketini işler, örn. "0 yorum" #}
{% fastcomments_comment_count %}

{# Canlı sohbet #}
{% fastcomments_live_chat %}

{# İşbirlikçi sohbet. Bir içerik öğesini bir CSS seçici ile hedefleyin #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Görsel sohbet. Bir resim öğesini bir CSS seçici ile hedefleyin #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# İncelemeler özeti #}
{% fastcomments_reviews_summary %}

{# Kullanıcı etkinlik akışı. Bir kullanıcı kimliği gerektirir #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Bir blog indeks sayfası için toplu yorum sayıları #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```