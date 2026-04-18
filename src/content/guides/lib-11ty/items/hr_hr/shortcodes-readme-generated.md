---
| Kratki kod | Opis |
| --- | --- |
| `fastcomments` | Widget za komentare s odgovorima, glasovanjem i ostalim značajkama |
| `fastcommentsCommentCount` | Prikazuje broj komentara za stranicu |
| `fastcommentsImageChat` | Komentari za označavanje slika |
| `fastcommentsLiveChat` | Widget za razgovor uživo |
| `fastcommentsCollabChat` | Surađivačko inline komentiranje |
| `fastcommentsRecentComments` | Nedavni komentari na cijelom web-mjestu |
| `fastcommentsRecentDiscussions` | Nedavno aktivne niti rasprave |
| `fastcommentsReviewsSummary` | Sažetak recenzija s ocjenama u zvjezdicama |
| `fastcommentsTopPages` | Najviše raspravljane stranice |
| `fastcommentsUserActivityFeed` | Feed aktivnosti korisnika |

### Primjeri

```njk
{# Broj komentara u tekstu #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Razgovor uživo #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Surađivački chat — ciljate element sadržaja pomoću CSS selektora #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Chat za slike — ciljanje elementa slike pomoću CSS selektora #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Sažetak recenzija #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Feed aktivnosti korisnika #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```
---