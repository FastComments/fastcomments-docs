---
| Shortcode | Description |
| --- | --- |
| `fastcomments` | Widget de commentaires avec réponses, votes et plus |
| `fastcommentsCommentCount` | Affiche le nombre de commentaires pour une page |
| `fastcommentsImageChat` | Commentaires d'annotation d'image |
| `fastcommentsLiveChat` | Widget de chat en direct |
| `fastcommentsCollabChat` | Commentaires collaboratifs en ligne |
| `fastcommentsRecentComments` | Commentaires récents sur l'ensemble du site |
| `fastcommentsRecentDiscussions` | Fils de discussion récemment actifs |
| `fastcommentsReviewsSummary` | Résumé des avis avec notation par étoiles |
| `fastcommentsTopPages` | Pages les plus discutées |
| `fastcommentsUserActivityFeed` | Fil d'activité utilisateur |

### Exemples

```njk
{# Nombre de commentaires intégré au texte #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Chat en direct #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Chat collaboratif — cibler un élément de contenu par sélecteur CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Chat d'image — cibler un élément image par sélecteur CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Résumé des avis #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Fil d'activité utilisateur #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```
---