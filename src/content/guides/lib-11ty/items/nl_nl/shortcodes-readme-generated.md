| Shortcode | Beschrijving |
| --- | --- |
| `fastcomments` | Reactie-widget met antwoorden, stemmen en meer |
| `fastcommentsCommentCount` | Toont het aantal reacties voor een pagina |
| `fastcommentsImageChat` | Annotatie-reacties voor afbeeldingen |
| `fastcommentsLiveChat` | Livechat-widget |
| `fastcommentsCollabChat` | Collaboratieve inlineopmerkingen |
| `fastcommentsRecentComments` | Recente reacties op de site |
| `fastcommentsRecentDiscussions` | Onlangs actieve discussiedraden |
| `fastcommentsReviewsSummary` | Samenvatting van sterrenbeoordelingen |
| `fastcommentsTopPages` | Meest besproken pagina's |
| `fastcommentsUserActivityFeed` | Gebruikersactiviteitenfeed |

### Voorbeelden

```njk
{# Reactieaantal inline met tekst #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Livechat #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Collab chat — wijs een contentelement aan met een CSS-selector #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Image chat — wijs een afbeeldingselement aan met een CSS-selector #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Samenvatting van beoordelingen #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Gebruikersactiviteitenfeed #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```