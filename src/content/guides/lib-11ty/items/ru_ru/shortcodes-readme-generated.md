| Шорткод | Описание |
| --- | --- |
| `fastcomments` | Виджет комментариев с ответами, голосованием и другими функциями |
| `fastcommentsCommentCount` | Отображает количество комментариев на странице |
| `fastcommentsImageChat` | Аннотации к изображениям (комментарии) |
| `fastcommentsLiveChat` | Виджет чата в реальном времени |
| `fastcommentsCollabChat` | Совместное встроенное комментирование |
| `fastcommentsRecentComments` | Недавние комментарии по всему сайту |
| `fastcommentsRecentDiscussions` | Недавно активные темы обсуждений |
| `fastcommentsReviewsSummary` | Сводка отзывов со звёздной оценкой |
| `fastcommentsTopPages` | Наиболее обсуждаемые страницы |
| `fastcommentsUserActivityFeed` | Лента активности пользователя |

### Примеры

```njk
{# Счётчик комментариев, встроенный в текст #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Чат в реальном времени #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Совместный чат — нацеливание на элемент контента по CSS-селектору #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Чат для изображений — нацеливание на элемент изображения по CSS-селектору #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Сводка отзывов #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Лента активности пользователя #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```