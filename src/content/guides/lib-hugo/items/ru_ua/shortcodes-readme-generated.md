---
| Шорткод | Описание |
| --- | --- |
| `fastcomments` | Дискуссионные комментарии с ответами, голосованием и упоминаниями (@mentions) |
| `fastcomments-comment-count` | Количество комментариев на одной странице |
| `fastcomments-comment-count-bulk` | Количество комментариев для многих страниц в одном запросе (см. [Массовые подсчёты комментариев](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Виджет живого чата |
| `fastcomments-collab-chat` | Коллаборативные встроенные комментарии (требует `target`) |
| `fastcomments-image-chat` | Комментарии с аннотациями изображений (требует `target`) |
| `fastcomments-recent-comments` | Последние комментарии по всему сайту |
| `fastcomments-recent-discussions` | Недавно активные темы обсуждений |
| `fastcomments-reviews-summary` | Сводка отзывов со звёздными оценками |
| `fastcomments-top-pages` | Наиболее обсуждаемые страницы |
| `fastcomments-user-activity-feed` | Лента активности пользователя (требует `userId`) |

### Примеры

Подсчёт комментариев встроено в текст:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Онлайн-чат:

```text
\{{< fastcomments-live-chat >}}
```

Коллаборативный чат, нацеливание на элемент контента с помощью CSS-селектора:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Чат для изображений, нацеливание на элемент изображения с помощью CSS-селектора:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Сводка отзывов:

```text
\{{< fastcomments-reviews-summary >}}
```

Лента активности пользователя:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```
---