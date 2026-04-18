| Shortcode | Opis |
| --- | --- |
| `fastcomments` | Widget za komentare sa odgovorima, glasovanjem i još mnogo toga |
| `fastcommentsCommentCount` | Prikazuje broj komentara za stranicu |
| `fastcommentsImageChat` | Komentari za anotiranje slika |
| `fastcommentsLiveChat` | Widget za live chat |
| `fastcommentsCollabChat` | Kolaborativno inline komentarisanje |
| `fastcommentsRecentComments` | Nedavni komentari na sajtu |
| `fastcommentsRecentDiscussions` | Nedavno aktivne teme diskusija |
| `fastcommentsReviewsSummary` | Sažetak recenzija sa ocjenama u zvjezdicama |
| `fastcommentsTopPages` | Najdiskutovanije stranice |
| `fastcommentsUserActivityFeed` | Feed aktivnosti korisnika |

### Primjeri

```njk
{# Broj komentara u tekstu #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Live chat #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Collab chat — ciljajte element sadržaja pomoću CSS selektora #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Image chat — ciljajte element slike pomoću CSS selektora #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Sažetak recenzija #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Feed aktivnosti korisnika #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```