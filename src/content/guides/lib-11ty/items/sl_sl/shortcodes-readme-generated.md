| Kratka koda | Opis |
| --- | --- |
| `fastcomments` | Pripomoček za komentiranje z odgovori, glasovanjem in drugimi funkcijami |
| `fastcommentsCommentCount` | Prikaže število komentarjev za stran |
| `fastcommentsImageChat` | Komentarji za označevanje slik |
| `fastcommentsLiveChat` | Pripomoček za klepet v živo |
| `fastcommentsCollabChat` | Sodelovalno vrstično komentiranje |
| `fastcommentsRecentComments` | Nedavni komentarji po spletnem mestu |
| `fastcommentsRecentDiscussions` | Nedavno aktivne niti razprav |
| `fastcommentsReviewsSummary` | Povzetek ocen z zvezdicami |
| `fastcommentsTopPages` | Strani z največ razprav |
| `fastcommentsUserActivityFeed` | Vir aktivnosti uporabnika |

### Primeri

```njk
{# Število komentarjev v vrstici z besedilom #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Klepet v živo #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Sodelovalni klepet — ciljanje elementa vsebine s CSS selektorjem #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Klepet za slike — ciljanje elementa slike s CSS selektorjem #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Povzetek ocen #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Vir aktivnosti uporabnika #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```