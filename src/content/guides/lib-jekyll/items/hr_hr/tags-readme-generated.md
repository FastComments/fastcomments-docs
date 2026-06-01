---
| Oznaka | Opis |
| --- | --- |
| `fastcomments` | Komentiranje uživo s odgovorima, glasovanjem, moderiranjem i ažuriranjima u stvarnom vremenu |
| `fastcomments_comment_count` | Broj komentara za trenutnu stranicu |
| `fastcomments_comment_count_bulk` | Brojevi komentara za više stranica na jednoj stranici popisa/indeksa |
| `fastcomments_live_chat` | Widget za chat u stvarnom vremenu |
| `fastcomments_collab_chat` | Suradničko inline komentiranje (tekstualne bilješke) |
| `fastcomments_image_chat` | Komentari s anotacijama na slikama |
| `fastcomments_recent_comments` | Nedavni komentari na cijelom web-mjestu |
| `fastcomments_recent_discussions` | Nedavno aktivne diskusije |
| `fastcomments_reviews_summary` | Sažetak recenzija sa zvjezdicama |
| `fastcomments_top_pages` | Stranice s najviše rasprava |
| `fastcomments_user_activity_feed` | Feed aktivnosti po korisniku |

### Primjeri

```liquid
{% raw %}{# Brojač komentara. Widget prikazuje svoju etiketu, npr. "0 komentara" #}
{% fastcomments_comment_count %}

{# Chat uživo #}
{% fastcomments_live_chat %}

{# Suradnički chat. Usmjerite ga na element sadržaja pomoću CSS selektora #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Chat za slike. Usmjerite ga na element slike pomoću CSS selektora #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Sažetak recenzija #}
{% fastcomments_reviews_summary %}

{# Feed korisničkih aktivnosti. Zahtijeva ID korisnika #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Masovni brojevi komentara za indeks bloga #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```