| Tag | Opis |
| --- | --- |
| `fastcomments` | Komentarisanje uživo sa odgovorima, glasovima, moderacijom i ažuriranjima u realnom vremenu |
| `fastcomments_comment_count` | Broj komentara za trenutnu stranicu |
| `fastcomments_comment_count_bulk` | Brojevi komentara za više stranica na jednoj listi/indeks stranici |
| `fastcomments_live_chat` | Widget za chat u realnom vremenu |
| `fastcomments_collab_chat` | Kolaborativno inline komentarisanje (tekstualne anotacije) |
| `fastcomments_image_chat` | Komentari za anotacije slika |
| `fastcomments_recent_comments` | Nedavni komentari na sajtu |
| `fastcomments_recent_discussions` | Nedavno aktivne teme diskusija |
| `fastcomments_reviews_summary` | Sažetak recenzija sa ocenama u zvezdicama |
| `fastcomments_top_pages` | Najdiskutovanije stranice |
| `fastcomments_user_activity_feed` | Feed aktivnosti po korisniku |

### Primeri

```liquid
{% raw %}{# Broj komentara. Widget prikazuje svoju oznaku, npr. "0 komentara" #}
{% fastcomments_comment_count %}

{# Chat uživo #}
{% fastcomments_live_chat %}

{# Kolaborativni chat. Usmerite ga na element sadržaja pomoću CSS selektora #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Chat za slike. Usmerite ga na element slike pomoću CSS selektora #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Sažetak recenzija #}
{% fastcomments_reviews_summary %}

{# Feed korisničkih aktivnosti. Zahteva ID korisnika #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Grupni brojevi komentara za indeks bloga #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```