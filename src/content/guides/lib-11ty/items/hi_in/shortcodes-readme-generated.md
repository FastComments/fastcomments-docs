| शॉर्टकोड | विवरण |
| --- | --- |
| `fastcomments` | जवाब, वोटिंग और बहुत कुछ के साथ कमेंटिंग विजेट |
| `fastcommentsCommentCount` | किसी पेज के लिए कमेंट काउंट दिखाता है |
| `fastcommentsImageChat` | इमेज एनोटेशन कमेंट्स |
| `fastcommentsLiveChat` | लाइव चैट विजेट |
| `fastcommentsCollabChat` | सहयोगात्मक इनलाइन कमेंटिंग |
| `fastcommentsRecentComments` | पूरी साइट के हाल के कमेंट्स |
| `fastcommentsRecentDiscussions` | हाल में सक्रिय चर्चा थ्रेड्स |
| `fastcommentsReviewsSummary` | स्टार-रेटिंग समीक्षा सारांश |
| `fastcommentsTopPages` | सबसे अधिक चर्चित पेज |
| `fastcommentsUserActivityFeed` | यूज़र गतिविधि फ़ीड |

### उदाहरण

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