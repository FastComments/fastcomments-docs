| Шорткод | Опис |
| --- | --- |
| `fastcomments` | Видгет за коментаре са одговорима, гласањем и још много тога |
| `fastcommentsCommentCount` | Приказује број коментара за страницу |
| `fastcommentsImageChat` | Коментари за анотацију слика |
| `fastcommentsLiveChat` | Видгет за разговор уживо |
| `fastcommentsCollabChat` | Колаборативно коментарисање унутар садржаја |
| `fastcommentsRecentComments` | Недавни коментари на целом сајту |
| `fastcommentsRecentDiscussions` | Недавно активне теме дискусија |
| `fastcommentsReviewsSummary` | Сажетак рецензија са звездицама |
| `fastcommentsTopPages` | Странице са највише дискусија |
| `fastcommentsUserActivityFeed` | Фид активности корисника |

### Примери

```njk
{# Број коментара у линији уз текст #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Разговор уживо #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Колаборативни чет — циљајте елемент садржаја помоћу CSS селектора #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Чет са сликом — циљајте елемент слике помоћу CSS селектора #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Сажетак рецензија #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Фид активности корисника #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```