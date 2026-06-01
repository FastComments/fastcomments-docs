---
| Tag | Description |
| --- | --- |
| `fastcomments` | Живая система комментариев с ответами, голосованием, модерацией и обновлениями в реальном времени |
| `fastcomments_comment_count` | Количество комментариев для текущей страницы |
| `fastcomments_comment_count_bulk` | Количество комментариев для многих страниц на одной странице списка/индекса |
| `fastcomments_live_chat` | Виджет потокового чата в режиме реального времени |
| `fastcomments_collab_chat` | Совместное встроенное комментирование (текстовые аннотации) |
| `fastcomments_image_chat` | Комментарии-аннотации к изображениям |
| `fastcomments_recent_comments` | Недавние комментарии по всему сайту |
| `fastcomments_recent_discussions` | Недавно активные темы обсуждений |
| `fastcomments_reviews_summary` | Сводка отзывов со звёздным рейтингом |
| `fastcomments_top_pages` | Самые обсуждаемые страницы |
| `fastcomments_user_activity_feed` | Лента активности по пользователю |

### Примеры

```liquid
{% raw %}{# Подсчёт комментариев. Виджет отображает собственную подпись, например "0 комментариев" #}
{% fastcomments_comment_count %}

{# Чат в реальном времени #}
{% fastcomments_live_chat %}

{# Collab chat. Укажите его для элемента контента с CSS-селектором #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Image chat. Укажите его для элемента изображения с CSS-селектором #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Сводка отзывов #}
{% fastcomments_reviews_summary %}

{# Лента активности пользователя. Требуется идентификатор пользователя #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Массовый подсчёт комментариев для индекса блога #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```
---