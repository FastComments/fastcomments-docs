| Shortcode | Description |
| --- | --- |
| `fastcomments` | Kommenteringswidget med svar, afstemning og mere |
| `fastcommentsCommentCount` | Visar antal kommentarer på en side |
| `fastcommentsImageChat` | Kommentering på billeder (annotering) |
| `fastcommentsLiveChat` | Livechat-widget |
| `fastcommentsCollabChat` | Samarbejdende inline-kommentering |
| `fastcommentsRecentComments` | Seneste kommentarer på tværs af siden |
| `fastcommentsRecentDiscussions` | For nylig aktive diskussionstråde |
| `fastcommentsReviewsSummary` | Oversigt over anmeldelser med stjernerating |
| `fastcommentsTopPages` | Mest diskuterede sider |
| `fastcommentsUserActivityFeed` | Brugeraktivitetsfeed |

### Eksempler

```njk
{# Kommentarantal inline med tekst #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Livechat #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Samarbejdschat — mål et indholdselement med en CSS-selektor #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Billedchat — mål et billedelement med en CSS-selektor #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Oversigt over anmeldelser #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Brugeraktivitetsfeed #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```