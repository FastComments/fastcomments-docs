---
| Σύντομος κώδικας | Περιγραφή |
| --- | --- |
| `fastcomments` | Widget σχολιασμού με απαντήσεις, ψηφοφορία και άλλα |
| `fastcommentsCommentCount` | Εμφανίζει τον αριθμό σχολίων για μια σελίδα |
| `fastcommentsImageChat` | Σχόλια με σημειώσεις σε εικόνες |
| `fastcommentsLiveChat` | Widget ζωντανής συνομιλίας |
| `fastcommentsCollabChat` | Συνεργατικός εντός-κειμένου σχολιασμός |
| `fastcommentsRecentComments` | Πρόσφατα σχόλια σε όλο το site |
| `fastcommentsRecentDiscussions` | Πρόσφατα ενεργά νήματα συζήτησης |
| `fastcommentsReviewsSummary` | Σύνοψη αξιολογήσεων με αστέρια |
| `fastcommentsTopPages` | Πιο συζητημένες σελίδες |
| `fastcommentsUserActivityFeed` | Ροή δραστηριότητας χρήστη |

### Παραδείγματα

```njk
{# Αριθμός σχολίων εντός κειμένου #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Ζωντανή συνομιλία #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Συνεργατική συνομιλία — στοχεύστε ένα στοιχείο περιεχομένου με επιλογέα CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Συνομιλία εικόνας — στοχεύστε ένα στοιχείο εικόνας με επιλογέα CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Σύνοψη αξιολογήσεων #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Ροή δραστηριότητας χρήστη #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```
---