Her widget'in kendi etiketi vardır. Hepsi `**extra` anahtar kelime argümanlarını kabul eder, bunlar widget yapılandırmasına olduğu gibi birleştirilir (camelCase anahtarlarını kullanın) aşağıdaki adlandırılmış argümanlar tarafından kapsanmayan her şey için.

| Tag | Widget |
|---|---|
| `{% fastcomments %}` | Yorumlar |
| `{% fastcomments_live_chat %}` | Canlı sohbet |
| `{% fastcomments_comment_count %}` | Yorum sayısı rozeti |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Toplu yorum sayıları |
| `{% fastcomments_collab_chat target="#el" %}` | İşbirlikçi (satır içi) sohbet |
| `{% fastcomments_image_chat target="#el" %}` | Görsel açıklama sohbeti |
| `{% fastcomments_recent_comments %}` | Son yorumlar |
| `{% fastcomments_recent_discussions %}` | Son tartışmalar |
| `{% fastcomments_reviews_summary %}` | İnceleme özeti |
| `{% fastcomments_top_pages %}` | En çok tartışılan sayfalar |
| `{% fastcomments_user_activity user_id="..." %}` | Bir kullanıcının etkinlik akışı |

Adlandırılmış argümanlar widget'ın camelCase yapılandırma anahtarlarına eşlenir:

| Argüman | Yapılandırma anahtarı | Etiketler |
|---|---|---|
| `url_id` | `urlId` | yorumlar, canlı sohbet, yorum sayısı, işbirlikçi/görsel sohbet, son yorumlar, inceleme özeti |
| `url` | `url` | yorumlar, canlı sohbet, işbirlikçi/görsel sohbet |
| `readonly` | `readonly` | yorumlar, canlı sohbet, işbirlikçi/görsel sohbet |
| `locale` | `locale` | yorumlar, canlı sohbet, işbirlikçi/görsel sohbet, kullanıcı etkinliği |
| `has_dark_background` | `hasDarkBackground` | tümü |
| `default_sort_direction` | `defaultSortDirection` | yorumlar, canlı sohbet, işbirlikçi/görsel sohbet |
| `number_only` | `numberOnly` | yorum sayısı |
| `is_live` | `isLive` | yorum sayısı |
| `count` | `count` | son yorumlar, son tartışmalar |
| `target` | (querySelector, gönderilmez) | işbirlikçi sohbet, görsel sohbet |
| `chat_square_percentage` | `chatSquarePercentage` | görsel sohbet |
| `user_id` | `userId` | kullanıcı etkinliği |

Örnekler:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# İşbirlikli sohbet sayfada mevcut bir öğeye eklenir #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Toplu sayımlar: işaretleyicileri yerleştirin, ardından tek bir toplu yükleyici hepsini doldurur #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```