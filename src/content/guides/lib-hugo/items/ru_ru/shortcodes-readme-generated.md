| Shortcode | Описание |
| --- | --- |
| `fastcomments` | Древовидные комментарии с ответами, голосованием и упоминаниями @ |
| `fastcomments-comment-count` | Количество комментариев на одной странице |
| `fastcomments-comment-count-bulk` | Количество комментариев для многих страниц в одном запросе (см. [Массовый подсчёт комментариев](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Виджет чата в реальном времени |
| `fastcomments-collab-chat` | Совместные встроенные комментарии (требует `target`) |
| `fastcomments-image-chat` | Комментарии-аннотации для изображений (требует `target`) |
| `fastcomments-recent-comments` | Недавние комментарии по всему сайту |
| `fastcomments-recent-discussions` | Недавно активные темы обсуждений |
| `fastcomments-reviews-summary` | Сводка отзывов со звёздным рейтингом |
| `fastcomments-top-pages` | Самые обсуждаемые страницы |
| `fastcomments-user-activity-feed` | Лента активности пользователя (требует `userId`) |

### Примеры

Счётчик комментариев в тексте:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Живой чат:

```text
\{{< fastcomments-live-chat >}}
```

Совместный чат, нацеленный на элемент содержимого по CSS-селектору:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Чат для изображений, нацеленный на элемент изображения по CSS-селектору:

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