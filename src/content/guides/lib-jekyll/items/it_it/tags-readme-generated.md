| Tag | Descrizione |
| --- | --- |
| `fastcomments` | Commenti in tempo reale con risposte, votazioni, moderazione e aggiornamenti in tempo reale |
| `fastcomments_comment_count` | Numero di commenti per la pagina corrente |
| `fastcomments_comment_count_bulk` | Conteggi dei commenti per molte pagine su una pagina elenco/indice |
| `fastcomments_live_chat` | Widget chat in tempo reale |
| `fastcomments_collab_chat` | Commenti collaborativi inline (annotazioni testuali) |
| `fastcomments_image_chat` | Commenti con annotazioni su immagini |
| `fastcomments_recent_comments` | Commenti recenti in tutto il sito |
| `fastcomments_recent_discussions` | Discussioni recentemente attive |
| `fastcomments_reviews_summary` | Riepilogo delle recensioni con valutazione a stelle |
| `fastcomments_top_pages` | Pagine più discusse |
| `fastcomments_user_activity_feed` | Feed attività per utente |

### Esempi

```liquid
{% raw %}{# Conteggio dei commenti. Il widget mostra la propria etichetta, es. "0 commenti" #}
{% fastcomments_comment_count %}

{# Chat in tempo reale #}
{% fastcomments_live_chat %}

{# Chat collaborativa. Puntala verso un elemento di contenuto con un selettore CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Chat per immagini. Puntala su un elemento immagine con un selettore CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Riepilogo delle recensioni #}
{% fastcomments_reviews_summary %}

{# Feed attività utente. Richiede un ID utente #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Conteggi dei commenti in blocco per un indice del blog #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```