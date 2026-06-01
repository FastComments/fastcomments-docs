| Shortcode | Descrizione |
| --- | --- |
| `fastcomments` | Commenti nidificati con risposte, voti e @menzioni |
| `fastcomments-comment-count` | Conteggio commenti per una singola pagina |
| `fastcomments-comment-count-bulk` | Conteggi dei commenti per più pagine in una sola richiesta (vedi [Conteggi commenti in blocco](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Widget chat in tempo reale |
| `fastcomments-collab-chat` | Commenti collaborativi inline (richiede `target`) |
| `fastcomments-image-chat` | Commenti con annotazioni sulle immagini (richiede `target`) |
| `fastcomments-recent-comments` | Commenti recenti in tutto il sito |
| `fastcomments-recent-discussions` | Discussioni attive recentemente |
| `fastcomments-reviews-summary` | Riepilogo delle recensioni con valutazione a stelle |
| `fastcomments-top-pages` | Pagine più discusse |
| `fastcomments-user-activity-feed` | Feed attività per utente (richiede `userId`) |

### Esempi

Conteggio commenti inline nel testo:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Chat in tempo reale:

```text
\{{< fastcomments-live-chat >}}
```

Chat collaborativa, mirata a un elemento di contenuto tramite selettore CSS:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Chat per immagini, mirata a un elemento immagine tramite selettore CSS:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Riepilogo recensioni:

```text
\{{< fastcomments-reviews-summary >}}
```

Feed attività per utente:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```