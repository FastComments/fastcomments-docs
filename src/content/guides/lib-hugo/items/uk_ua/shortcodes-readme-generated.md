| Шорткод | Опис |
| --- | --- |
| `fastcomments` | Вкладені коментарі з відповідями, голосуванням та @згадуваннями |
| `fastcomments-comment-count` | Кількість коментарів для однієї сторінки |
| `fastcomments-comment-count-bulk` | Кількості коментарів для багатьох сторінок в одному запиті (див. [Масові підрахунки коментарів](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Віджет живого чату |
| `fastcomments-collab-chat` | Спільне інтегроване коментування (вимагає `target`) |
| `fastcomments-image-chat` | Коментарі з анотацією зображень (вимагає `target`) |
| `fastcomments-recent-comments` | Останні коментарі по всьому сайту |
| `fastcomments-recent-discussions` | Нещодавно активні дискусії |
| `fastcomments-reviews-summary` | Підсумок відгуків зі зірковим рейтингом |
| `fastcomments-top-pages` | Найбільш обговорювані сторінки |
| `fastcomments-user-activity-feed` | Стрічка активності користувача (вимагає `userId`) |

### Приклади

Підрахунок коментарів у рядку з текстом:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Живий чат:

```text
\{{< fastcomments-live-chat >}}
```

Спільний чат — націлювання елемента вмісту за допомогою CSS-селектора:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Чат для зображень — націлювання елемента зображення за допомогою CSS-селектора:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Підсумок відгуків:

```text
\{{< fastcomments-reviews-summary >}}
```

Стрічка активності користувача:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```
---