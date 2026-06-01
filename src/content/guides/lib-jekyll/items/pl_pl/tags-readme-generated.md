| Tag | Opis |
| --- | --- |
| `fastcomments` | Dyskusje na żywo z odpowiedziami, głosowaniem, moderacją i aktualizacjami w czasie rzeczywistym |
| `fastcomments_comment_count` | Liczba komentarzy dla bieżącej strony |
| `fastcomments_comment_count_bulk` | Liczby komentarzy dla wielu stron na jednej stronie listy/indeksu |
| `fastcomments_live_chat` | Widżet czatu strumieniowego w czasie rzeczywistym |
| `fastcomments_collab_chat` | Współpraca przy komentarzach inline (adnotacje tekstowe) |
| `fastcomments_image_chat` | Komentarze z adnotacjami obrazów |
| `fastcomments_recent_comments` | Najnowsze komentarze w całej witrynie |
| `fastcomments_recent_discussions` | Niedawno aktywne wątki dyskusji |
| `fastcomments_reviews_summary` | Podsumowanie recenzji (oceny w gwiazdkach) |
| `fastcomments_top_pages` | Najczęściej dyskutowane strony |
| `fastcomments_user_activity_feed` | Kanał aktywności dla użytkownika |

### Przykłady

```liquid
{% raw %}{# Liczba komentarzy. Widżet renderuje własną etykietę, np. "0 komentarzy" #}
{% fastcomments_comment_count %}

{# Czat na żywo #}
{% fastcomments_live_chat %}

{# Collab chat. Wskaż go na element treści za pomocą selektora CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Image chat. Wskaż go na element obrazu za pomocą selektora CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Podsumowanie recenzji #}
{% fastcomments_reviews_summary %}

{# Kanał aktywności użytkownika. Wymaga identyfikatora użytkownika #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Zbiorcze liczby komentarzy dla indeksu bloga #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```