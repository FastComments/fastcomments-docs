| Кратки код | Опис |
| --- | --- |
| `fastcomments` | Нитовани коментари са одговорима, гласањем и @поменама |
| `fastcomments-comment-count` | Број коментара за једну страницу |
| `fastcomments-comment-count-bulk` | Бројеви коментара за више страница у једном захтјеву (види [Бројеви коментара у серији](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Видгет за ћаскање уживо |
| `fastcomments-collab-chat` | Сарадничко инлајн коментарисање (захтијева `target`) |
| `fastcomments-image-chat` | Коментари за анотацију слика (захтијева `target`) |
| `fastcomments-recent-comments` | Недавни коментари широм сајта |
| `fastcomments-recent-discussions` | Недавно активне дискусионе нити |
| `fastcomments-reviews-summary` | Сажетак оцјена са звјездицама |
| `fastcomments-top-pages` | Странице са највише дискусија |
| `fastcomments-user-activity-feed` | Фид активности по кориснику (захтијева `userId`) |

### Примери

Број коментара уграђен у текст:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Ћаскање уживо:

```text
\{{< fastcomments-live-chat >}}
```

Сарадничко ћаскање, циљање елемента садржаја помоћу CSS селектора:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Часкање за слике, циљање елемента слике помоћу CSS селектора:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Сажетак оцјена:

```text
\{{< fastcomments-reviews-summary >}}
```

Фид активности корисника:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```