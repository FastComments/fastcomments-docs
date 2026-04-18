---
| Συντομοκώδικας | Περιγραφή |
| --- | --- |
| `fastcomments` | Εργαλείο σχολιασμού με απαντήσεις, ψήφους και άλλα |
| `fastcommentsCommentCount` | Εμφανίζει το πλήθος σχολίων για μια σελίδα |
| `fastcommentsImageChat` | Σχολιασμός εικόνας |
| `fastcommentsLiveChat` | Widget ζωντανής συνομιλίας |
| `fastcommentsCollabChat` | Συνεργατικός ενσωματωμένος σχολιασμός |
| `fastcommentsRecentComments` | Πρόσφατα σχόλια σε όλο τον ιστότοπο |
| `fastcommentsRecentDiscussions` | Πρόσφατα ενεργά νήματα συζήτησης |
| `fastcommentsReviewsSummary` | Σύνοψη αξιολογήσεων με αστέρια |
| `fastcommentsTopPages` | Πιο συζητημένες σελίδες |
| `fastcommentsUserActivityFeed` | Ροή δραστηριότητας χρήστη |

### Παραδείγματα

```njk
{# Πλήθος σχολίων εντός του κειμένου #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Ζωντανή συνομιλία #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Συνεργατική συνομιλία — στοχεύστε ένα στοιχείο περιεχομένου χρησιμοποιώντας επιλογέα CSS #}
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