| Шорткод | Описание |
| --- | --- |
| `fastcomments` | Уиджет за коментари с отговори, гласуване и още |
| `fastcommentsCommentCount` | Показва броя коментари за страница |
| `fastcommentsImageChat` | Коментари за анотиране на изображения |
| `fastcommentsLiveChat` | Уиджет за чат на живо |
| `fastcommentsCollabChat` | Съвместно вградено коментиране |
| `fastcommentsRecentComments` | Последни коментари в целия сайт |
| `fastcommentsRecentDiscussions` | Наскоро активни нишки на дискусии |
| `fastcommentsReviewsSummary` | Обобщение на ревюта със звездни оценки |
| `fastcommentsTopPages` | Най-обсъжданите страници |
| `fastcommentsUserActivityFeed` | Лента за активността на потребителя |

### Примери

```njk
{# Брой коментари в ред с текста #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Чат на живо #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Съвместен чат — насочване към елемент от съдържанието чрез CSS селектор #}
<article id="post-body">
  <p>Маркирайте ме, за да оставите коментар.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Чат за изображения — насочване към елемент изображение чрез CSS селектор #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Обобщение на ревютата #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Лента за активността на потребителя #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```