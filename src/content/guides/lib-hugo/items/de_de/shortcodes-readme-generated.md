| Shortcode | Beschreibung |
| --- | --- |
| `fastcomments` | Verschachtelte Kommentare mit Antworten, Abstimmungen und @-Erwähnungen |
| `fastcomments-comment-count` | Anzahl der Kommentare für eine einzelne Seite |
| `fastcomments-comment-count-bulk` | Kommentaranzahlen für viele Seiten in einer Anfrage (siehe [Massen-Kommentaranzahlen](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Live-Chat-Widget |
| `fastcomments-collab-chat` | Kollaborative Inline-Kommentare (erfordert `target`) |
| `fastcomments-image-chat` | Bildannotation-Kommentare (erfordert `target`) |
| `fastcomments-recent-comments` | Aktuelle Kommentare auf der gesamten Website |
| `fastcomments-recent-discussions` | Kürzlich aktive Diskussionsstränge |
| `fastcomments-reviews-summary` | Zusammenfassung der Sternbewertungen |
| `fastcomments-top-pages` | Meistdiskutierte Seiten |
| `fastcomments-user-activity-feed` | Aktivitäts-Feed pro Benutzer (erfordert `userId`) |

### Beispiele

Kommentaranzahl inline im Text:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Live-Chat:

```text
\{{< fastcomments-live-chat >}}
```

Kollaborativer Chat, der ein Inhaltselement per CSS-Selektor anspricht:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Bild-Chat, der ein Bildelement per CSS-Selektor anspricht:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Zusammenfassung der Bewertungen:

```text
\{{< fastcomments-reviews-summary >}}
```

Aktivitäts-Feed eines Benutzers:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```