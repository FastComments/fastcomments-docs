| Shortcode | Description |
| --- | --- |
| `fastcomments` | Коментари у низу са одговорима, гласањем и @поменама |
| `fastcomments-comment-count` | Број коментара за једну страницу |
| `fastcomments-comment-count-bulk` | Бројеви коментара за више страница у једном захтеву (погледајте [Групни бројеви коментара](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Видгет за разговор уживо |
| `fastcomments-collab-chat` | Колаборативно инлајн коментарисање (захтева `target`) |
| `fastcomments-image-chat` | Коментари са анотацијом слика (захтева `target`) |
| `fastcomments-recent-comments` | Најновији коментари на целом сајту |
| `fastcomments-recent-discussions` | Недавно активне дискусије |
| `fastcomments-reviews-summary` | Сажетак рецензија са звездицама |
| `fastcomments-top-pages` | Странице са највише дискусија |
| `fastcomments-user-activity-feed` | Фид активности по кориснику (захтева `userId`) |

### Примери

Број коментара у тексту:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Разговор уживо:

```text
\{{< fastcomments-live-chat >}}
```

Колаборативни чет, циљање елемента садржаја помоћу CSS селектора:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Чет за слике, циљање елемента слике помоћу CSS селектора:

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