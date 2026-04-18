---
| Shortcode | Описание |
| --- | --- |
| `fastcomments` | Виджет комментариев с ответами, голосованием и другими возможностями |
| `fastcommentsCommentCount` | Отображает количество комментариев на странице |
| `fastcommentsImageChat` | Комментарии-аннотации для изображений |
| `fastcommentsLiveChat` | Виджет чата в реальном времени |
| `fastcommentsCollabChat` | Совместное встроенное комментирование |
| `fastcommentsRecentComments` | Последние комментарии по всему сайту |
| `fastcommentsRecentDiscussions` | Недавно активные темы обсуждений |
| `fastcommentsReviewsSummary` | Сводка отзывов со звездным рейтингом |
| `fastcommentsTopPages` | Самые обсуждаемые страницы |
| `fastcommentsUserActivityFeed` | Лента активности пользователя |

### Примеры

```njk
{# Количество комментариев в тексте #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Чат в реальном времени #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Совместный чат — укажите элемент контента с помощью CSS-селектора #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Чат для изображений — укажите элемент изображения с помощью CSS-селектора #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Сводка отзывов #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Лента активности пользователя #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```
---