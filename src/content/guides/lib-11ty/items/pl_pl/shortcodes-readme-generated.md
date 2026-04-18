---
| Shortcode | Opis |
| --- | --- |
| `fastcomments` | Widget komentarzy z możliwością odpowiedzi, głosowania i innymi funkcjami |
| `fastcommentsCommentCount` | Wyświetla liczbę komentarzy na stronie |
| `fastcommentsImageChat` | Komentarze z adnotacjami obrazów |
| `fastcommentsLiveChat` | Widget czatu na żywo |
| `fastcommentsCollabChat` | Wspólne komentowanie inline |
| `fastcommentsRecentComments` | Ostatnie komentarze w całej witrynie |
| `fastcommentsRecentDiscussions` | Ostatnio aktywne wątki dyskusyjne |
| `fastcommentsReviewsSummary` | Podsumowanie recenzji z ocenami gwiazdkowymi |
| `fastcommentsTopPages` | Najczęściej dyskutowane strony |
| `fastcommentsUserActivityFeed` | Kanał aktywności użytkownika |

### Przykłady

```njk
{# Liczba komentarzy w tekście #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Czat na żywo #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Collab chat — skieruj do elementu treści za pomocą selektora CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Image chat — skieruj do elementu obrazu za pomocą selektora CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Podsumowanie recenzji #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Kanał aktywności użytkownika #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```
---