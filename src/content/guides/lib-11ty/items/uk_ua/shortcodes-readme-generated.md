| Shortcode | Description |
| --- | --- |
| `fastcomments` | Віджет коментарів із відповідями, голосуванням та інше |
| `fastcommentsCommentCount` | Відображає кількість коментарів на сторінці |
| `fastcommentsImageChat` | Коментарі-анотації для зображень |
| `fastcommentsLiveChat` | Віджет живого чату |
| `fastcommentsCollabChat` | Спільні вбудовані коментарі |
| `fastcommentsRecentComments` | Останні коментарі на сайті |
| `fastcommentsRecentDiscussions` | Нещодавно активні дискусії |
| `fastcommentsReviewsSummary` | Підсумок відгуків зі зірковими оцінками |
| `fastcommentsTopPages` | Найбільш обговорювані сторінки |
| `fastcommentsUserActivityFeed` | Стрічка активності користувача |

### Приклади

```njk
{# Лічильник коментарів вбудовано в текст #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Живий чат #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Collab chat — націлити елемент контенту за допомогою CSS-селектора #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Image chat — націлити елемент зображення за допомогою CSS-селектора #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Підсумок відгуків #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Стрічка активності користувача #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```