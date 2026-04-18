---
| Shortcode | Beschreibung |
| --- | --- |
| `fastcomments` | Kommentierungs-Widget mit Antworten, Abstimmungen und mehr |
| `fastcommentsCommentCount` | Zeigt die Anzahl der Kommentare für eine Seite an |
| `fastcommentsImageChat` | Kommentare zur Bildannotation |
| `fastcommentsLiveChat` | Live-Chat-Widget |
| `fastcommentsCollabChat` | Kollaborative Inline-Kommentare |
| `fastcommentsRecentComments` | Neueste Kommentare auf der gesamten Website |
| `fastcommentsRecentDiscussions` | Kürzlich aktive Diskussionsthreads |
| `fastcommentsReviewsSummary` | Zusammenfassung der Sternebewertungen |
| `fastcommentsTopPages` | Am häufigsten diskutierte Seiten |
| `fastcommentsUserActivityFeed` | Benutzeraktivitäts-Feed |

### Beispiele

```njk
{# Kommentaranzahl inline im Text #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Live-Chat #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Collab-Chat — ein Inhaltselement mit einem CSS-Selektor anvisieren #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Image-Chat — ein Bildelement mit einem CSS-Selektor anvisieren #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Zusammenfassung der Bewertungen #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Benutzeraktivitäts-Feed #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```
---