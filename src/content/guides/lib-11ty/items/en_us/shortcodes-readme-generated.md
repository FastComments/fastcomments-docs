| Shortcode | Description |
| --- | --- |
| `fastcomments` | Commenting widget with replies, voting, and more |
| `fastcommentsCommentCount` | Displays comment count for a page |
| `fastcommentsImageChat` | Image annotation comments |
| `fastcommentsLiveChat` | Live chat widget |
| `fastcommentsCollabChat` | Collaborative inline commenting |
| `fastcommentsRecentComments` | Recent comments across the site |
| `fastcommentsRecentDiscussions` | Recently active discussion threads |
| `fastcommentsReviewsSummary` | Star-rating reviews summary |
| `fastcommentsTopPages` | Most-discussed pages |
| `fastcommentsUserActivityFeed` | User activity feed |

### Examples

```njk
{# Comment count inline with text #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Live chat #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Collab chat — target a content element by CSS selector #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Image chat — target an image element by CSS selector #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Reviews summary #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# User activity feed #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```