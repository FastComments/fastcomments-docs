| Кратки код | Опис |
| --- | --- |
| `fastcomments` | Видгет за коментаре са одговорима, гласањем и још много тога |
| `fastcommentsCommentCount` | Приказује број коментара за страницу |
| `fastcommentsImageChat` | Коментари за анотацију слика |
| `fastcommentsLiveChat` | Видгет за чет уживо |
| `fastcommentsCollabChat` | Колаборативно унутарлинерско коментарисање |
| `fastcommentsRecentComments` | Недавни коментари широм сајта |
| `fastcommentsRecentDiscussions` | Недавно активне дискусије |
| `fastcommentsReviewsSummary` | Резиме рецензија са оцјенама у звездама |
| `fastcommentsTopPages` | Највише дискутоване странице |
| `fastcommentsUserActivityFeed` | Фид активности корисника |

### Примери

```njk
{# Број коментара у линији са текстом #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Чет уживо #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Колаб чет — циљајте елемент садржаја помоћу CSS селектора #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Чет на слици — циљајте елемент слике помоћу CSS селектора #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Резиме рецензија #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Фид активности корисника #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```