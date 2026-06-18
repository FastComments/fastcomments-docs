| Kratka koda | Opis |
| --- | --- |
| `fastcomments` | Večnivovski komentarji z odgovori, glasovanjem in omembami (@mentions) |
| `fastcomments-comment-count` | Število komentarjev za posamezno stran |
| `fastcomments-comment-count-bulk` | Število komentarjev za več strani v eni zahtevi (glej [Masovna številanja komentarjev](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Klepet v živo |
| `fastcomments-collab-chat` | Sodelovalno vnosno komentiranje (zahteva `target`) |
| `fastcomments-image-chat` | Komentarji za označevanje slik (zahteva `target`) |
| `fastcomments-recent-comments` | Nedavni komentarji po spletnem mestu |
| `fastcomments-recent-discussions` | Nedavno aktivne niti razprav |
| `fastcomments-reviews-summary` | Povzetek ocen z zvezdicami |
| `fastcomments-top-pages` | Najbolj razpravljane strani |
| `fastcomments-user-activity-feed` | Vir aktivnosti posameznega uporabnika (zahteva `userId`) |

### Primeri

Štetje komentarjev v besedilu:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Klepet v živo:

```text
\{{< fastcomments-live-chat >}}
```

Sodelovalni klepet, ciljanje elementa vsebine s CSS selektorjem:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Klepet za slike, ciljanje elementa slike s CSS selektorjem:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Povzetek ocen:

```text
\{{< fastcomments-reviews-summary >}}
```

Vir aktivnosti uporabnika:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```