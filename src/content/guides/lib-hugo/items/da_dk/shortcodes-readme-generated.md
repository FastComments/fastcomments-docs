| Shortcode | Beskrivelse |
| --- | --- |
| `fastcomments` | Trådede kommentarer med svar, afstemning og @mentions |
| `fastcomments-comment-count` | Kommentarantal for en enkelt side |
| `fastcomments-comment-count-bulk` | Kommentarantal for mange sider i én forespørgsel (se [Bulk-kommentaroptællinger](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Live chat-widget |
| `fastcomments-collab-chat` | Samarbejdende inline-kommentarer (kræver `target`) |
| `fastcomments-image-chat` | Kommentarer til billedannotering (kræver `target`) |
| `fastcomments-recent-comments` | Seneste kommentarer på tværs af webstedet |
| `fastcomments-recent-discussions` | Nyligt aktive diskussionstråde |
| `fastcomments-reviews-summary` | Oversigt over stjerneanmeldelser |
| `fastcomments-top-pages` | Mest diskuterede sider |
| `fastcomments-user-activity-feed` | Bruger-specifikt aktivitetsfeed (kræver `userId`) |

### Eksempler

Kommentarantal inline med tekst:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Live chat:

```text
\{{< fastcomments-live-chat >}}
```

Collab chat, målretter et indholdselement med en CSS-selektor:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Image chat, målretter et billedelement med en CSS-selektor:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Anmeldelsesoversigt:

```text
\{{< fastcomments-reviews-summary >}}
```

Brugeraktivitetsfeed:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```