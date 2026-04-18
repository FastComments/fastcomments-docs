| Shortcode | Descrizione |
| --- | --- |
| `fastcomments` | Widget di commenti con risposte, votazioni e altro |
| `fastcommentsCommentCount` | Mostra il conteggio dei commenti per una pagina |
| `fastcommentsImageChat` | Commenti con annotazioni sulle immagini |
| `fastcommentsLiveChat` | Widget di chat in tempo reale |
| `fastcommentsCollabChat` | Commenti inline collaborativi |
| `fastcommentsRecentComments` | Commenti recenti su tutto il sito |
| `fastcommentsRecentDiscussions` | Discussioni attive di recente |
| `fastcommentsReviewsSummary` | Riepilogo delle recensioni con valutazione a stelle |
| `fastcommentsTopPages` | Pagine più discusse |
| `fastcommentsUserActivityFeed` | Feed attività utente |

### Esempi

```njk
{# Conteggio dei commenti inline nel testo #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Chat live #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Chat collaborativa — seleziona un elemento di contenuto tramite selettore CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Chat immagine — seleziona un elemento immagine tramite selettore CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Riepilogo recensioni #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Feed attività utente #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```