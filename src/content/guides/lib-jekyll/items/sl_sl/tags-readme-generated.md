| Oznaka | Opis |
| --- | --- |
| `fastcomments` | Komentiranje v živo z odgovori, glasovanjem, moderiranjem in posodobitvami v realnem času |
| `fastcomments_comment_count` | Število komentarjev za trenutno stran |
| `fastcomments_comment_count_bulk` | Števila komentarjev za več strani na eni seznamni/indeksni strani |
| `fastcomments_live_chat` | Vtičnik za klepet v realnem času |
| `fastcomments_collab_chat` | Sodelovalno vdelano komentiranje (besedilne anotacije) |
| `fastcomments_image_chat` | Komentarji z anotacijami na sliki |
| `fastcomments_recent_comments` | Nedavni komentarji po celotnem spletnem mestu |
| `fastcomments_recent_discussions` | Nedavno aktivne razprave |
| `fastcomments_reviews_summary` | Povzetek ocen z zvezdicami |
| `fastcomments_top_pages` | Najbolj razpravljane strani |
| `fastcomments_user_activity_feed` | Vir aktivnosti posameznega uporabnika |

### Primeri

```liquid
{% raw %}{# Število komentarjev. Vtičnik prikaže svojo oznako, npr. "0 komentarjev" #}
{% fastcomments_comment_count %}

{# Klepet v živo #}
{% fastcomments_live_chat %}

{# Sodelovalni klepet. Usmerite ga na element vsebine z CSS selektorjem #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Klepet za slike. Usmerite ga na element slike z CSS selektorjem #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Povzetek ocen #}
{% fastcomments_reviews_summary %}

{# Vir aktivnosti uporabnika. Zahteva ID uporabnika #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Števila komentarjev za več objav na indeksni strani bloga #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```