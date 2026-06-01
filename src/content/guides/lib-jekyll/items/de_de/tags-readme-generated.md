| Tag | Beschreibung |
| --- | --- |
| `fastcomments` | Echtzeit-Kommentare mit Antworten, Abstimmungen, Moderation und Live-Aktualisierungen |
| `fastcomments_comment_count` | Anzahl der Kommentare für die aktuelle Seite |
| `fastcomments_comment_count_bulk` | Kommentaranzahlen für viele Seiten auf einer Listen-/Indexseite |
| `fastcomments_live_chat` | Echtzeit-Streaming-Chat-Widget |
| `fastcomments_collab_chat` | Kollaborative Inline-Kommentare (Textannotationen) |
| `fastcomments_image_chat` | Kommentare für Bildannotationen |
| `fastcomments_recent_comments` | Aktuelle Kommentare auf der gesamten Website |
| `fastcomments_recent_discussions` | Kürzlich aktive Diskussionen |
| `fastcomments_reviews_summary` | Zusammenfassung der Sternebewertungen |
| `fastcomments_top_pages` | Am meisten diskutierte Seiten |
| `fastcomments_user_activity_feed` | Benutzerspezifischer Aktivitätsfeed |

### Beispiele

```liquid
{% raw %}{# Kommentaranzahl. Das Widget rendert sein eigenes Label, z. B. "0 Kommentare" #}
{% fastcomments_comment_count %}

{# Live-Chat #}
{% fastcomments_live_chat %}

{# Kollaborativer Chat. Richte ihn auf ein Inhaltselement mit einem CSS-Selektor aus #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Bild-Chat. Richte ihn auf ein Bildelement mit einem CSS-Selektor aus #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Zusammenfassung der Bewertungen #}
{% fastcomments_reviews_summary %}

{# Benutzer-Aktivitätsfeed. Erfordert eine Benutzer-ID #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Sammel-Kommentaranzahlen für einen Blog-Index #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```