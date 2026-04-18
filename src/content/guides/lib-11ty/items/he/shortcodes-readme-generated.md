| קוד מקוצר | תיאור |
| --- | --- |
| `fastcomments` | ווידג'ט תגובות עם תגובות משנה, הצבעות ועוד |
| `fastcommentsCommentCount` | מציג את מספר התגובות עבור עמוד |
| `fastcommentsImageChat` | הערות על תמונות |
| `fastcommentsLiveChat` | ווידג'ט צ'אט חי |
| `fastcommentsCollabChat` | הערות שיתופיות داخل הטקסט |
| `fastcommentsRecentComments` | תגובות אחרונות בכל האתר |
| `fastcommentsRecentDiscussions` | שרשורי דיון שהיו פעילים לאחרונה |
| `fastcommentsReviewsSummary` | סיכום ביקורות בדירוג כוכבים |
| `fastcommentsTopPages` | העמודים הנדונים ביותר |
| `fastcommentsUserActivityFeed` | פיד פעילות משתמש |

### דוגמאות

```njk
{# ספירת תגובות בתוך הטקסט #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# צ'אט חי #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# צ'אט שיתופי — ייעד אלמנט תוכן באמצעות בורר CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# צ'אט תמונה — ייעד אלמנט תמונה באמצעות בורר CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# סיכום ביקורות #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# פיד פעילות משתמש #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```