| Shortcode | Opis |
| --- | --- |
| `fastcomments` | Widget za komentare sa odgovorima, glasovima i još mnogo toga |
| `fastcommentsCommentCount` | Prikazuje broj komentara za stranicu |
| `fastcommentsImageChat` | Komentari za označavanje slika |
| `fastcommentsLiveChat` | Widget za ćaskanje uživo |
| `fastcommentsCollabChat` | Kolaborativno inline komentarisanje |
| `fastcommentsRecentComments` | Najnoviji komentari na sajtu |
| `fastcommentsRecentDiscussions` | Nedavno aktivne diskusije |
| `fastcommentsReviewsSummary` | Sažetak ocena sa zvezdicama |
| `fastcommentsTopPages` | Najviše diskutovane stranice |
| `fastcommentsUserActivityFeed` | Tok aktivnosti korisnika |

### Primeri

```njk
{# Brojač komentara u tekstu #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Ćaskanje uživo #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Kolaborativni chat — ciljajte element sadržaja pomoću CSS selektora #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Chat za slike — ciljajte element slike pomoću CSS selektora #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Sažetak recenzija #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Feed aktivnosti korisnika #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```