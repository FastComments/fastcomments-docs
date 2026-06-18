| Кратък код | Описание |
| --- | --- |
| `fastcomments` | Коментари с нишки с отговори, гласуване и @споменавания |
| `fastcomments-comment-count` | Брой коментари за една страница |
| `fastcomments-comment-count-bulk` | Брой коментари за много страници в един заявка (вж. [Брой коментари в масов режим](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Уиджет за чат на живо |
| `fastcomments-collab-chat` | Съвместно вградени коментари (изисква `target`) |
| `fastcomments-image-chat` | Коментари за анотация на изображения (изисква `target`) |
| `fastcomments-recent-comments` | Последни коментари в целия сайт |
| `fastcomments-recent-discussions` | Наскоро активни дискусионни нишки |
| `fastcomments-reviews-summary` | Резюме на ревюта с оценка със звезди |
| `fastcomments-top-pages` | Най-обсъжданите страници |
| `fastcomments-user-activity-feed` | Фийд с активността на потребителя (изисква `userId`) |

### Примери

Брой коментари в текста:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Чат на живо:

```text
\{{< fastcomments-live-chat >}}
```

Съвместен чат, насочен към елемент от съдържанието чрез CSS селектор:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Чат за изображения, насочен към елемент изображение чрез CSS селектор:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Резюме на ревюта:

```text
\{{< fastcomments-reviews-summary >}}
```

Фийд с активността на потребителя:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```