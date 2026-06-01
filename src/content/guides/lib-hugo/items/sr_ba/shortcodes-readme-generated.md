---
| Кратки код | Опис |
| --- | --- |
| `fastcomments` | Коментари у нитима са одговорима, гласањем и @поменама |
| `fastcomments-comment-count` | Број коментара за једну страницу |
| `fastcomments-comment-count-bulk` | Бројеви коментара за више страница у једном захтјеву (погледајте [Групни бројеви коментара](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Видгет за ћаскање уживо |
| `fastcomments-collab-chat` | Колаборативно коментарисање у линији (захтијева `target`) |
| `fastcomments-image-chat` | Коментари за анотације слике (захтијева `target`) |
| `fastcomments-recent-comments` | Најновији коментари широм сајта |
| `fastcomments-recent-discussions` | Недавно активни тредови дискусије |
| `fastcomments-reviews-summary` | Сажетак оцјена са звјездицама |
| `fastcomments-top-pages` | Најдискусираније странице |
| `fastcomments-user-activity-feed` | Фид активности по кориснику (захтијева `userId`) |

### Примјери

Број коментара у линији са текстом:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Ћаскање уживо:

```text
\{{< fastcomments-live-chat >}}
```

Колаборативни чат, циљање елемента садржаја помоћу CSS селектора:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Чат за слике, циљање елемента слике помоћу CSS селектора:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Сажетак рецензија:

```text
\{{< fastcomments-reviews-summary >}}
```

Фид активности корисника:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```
---