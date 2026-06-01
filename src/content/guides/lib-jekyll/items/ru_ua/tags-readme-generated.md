| Тег | Описание |
| --- | --- |
| `fastcomments` | Живые комментарии с ответами, голосованием, модерацией и обновлениями в реальном времени |
| `fastcomments_comment_count` | Количество комментариев для текущей страницы |
| `fastcomments_comment_count_bulk` | Количество комментариев для многих страниц на одной странице списка/индекса |
| `fastcomments_live_chat` | Виджет чата с потоковой передачей в реальном времени |
| `fastcomments_collab_chat` | Совместная встроенная система комментирования (текстовые аннотации) |
| `fastcomments_image_chat` | Комментарии с аннотацией изображений |
| `fastcomments_recent_comments` | Недавние комментарии по всему сайту |
| `fastcomments_recent_discussions` | Недавно активные темы обсуждений |
| `fastcomments_reviews_summary` | Сводка отзывов со звездным рейтингом |
| `fastcomments_top_pages` | Наиболее обсуждаемые страницы |
| `fastcomments_user_activity_feed` | Лента активности пользователя |

### Examples

```liquid
{% raw %}{# Счетчик комментариев. Виджет отображает собственную подпись, например "0 комментариев" #}
{% fastcomments_comment_count %}

{# Чат в реальном времени #}
{% fastcomments_live_chat %}

{# Совместный чат. Укажите элемент контента с помощью CSS-селектора #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Чат для изображений. Укажите элемент изображения с помощью CSS-селектора #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Сводка отзывов #}
{% fastcomments_reviews_summary %}

{# Лента активности пользователя. Требуется user id #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Массовые счётчики комментариев для индексной страницы блога #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```