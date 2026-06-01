| Тег | Опис |
| --- | --- |
| `fastcomments` | Живі коментарі з відповідями, голосуванням, модеруванням та оновленнями в реальному часі |
| `fastcomments_comment_count` | Кількість коментарів для поточної сторінки |
| `fastcomments_comment_count_bulk` | Кількості коментарів для кількох сторінок на одній сторінці списку/індексу |
| `fastcomments_live_chat` | Потоковий чат у реальному часі |
| `fastcomments_collab_chat` | Спільне вбудоване коментування (текстові анотації) |
| `fastcomments_image_chat` | Коментарі-анотації до зображень |
| `fastcomments_recent_comments` | Останні коментарі по всьому сайту |
| `fastcomments_recent_discussions` | Нещодавно активні теми обговорень |
| `fastcomments_reviews_summary` | Підсумок оглядів із зірковими рейтингами |
| `fastcomments_top_pages` | Найбільш обговорювані сторінки |
| `fastcomments_user_activity_feed` | Стрічка активності користувача |

### Приклади

```liquid
{% raw %}{# Кількість коментарів. Віджет відображає власну мітку, наприклад "0 коментарів" #}
{% fastcomments_comment_count %}

{# Чат у реальному часі #}
{% fastcomments_live_chat %}

{# Спільне вбудоване коментування. Вкажіть селектором CSS елемент вмісту #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Чат для зображень. Вкажіть його на елемент зображення за допомогою CSS-селектора #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Підсумок оглядів #}
{% fastcomments_reviews_summary %}

{# Стрічка активності користувача. Потребує ідентифікатора користувача #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Масові підрахунки коментарів для індексу блогу #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```