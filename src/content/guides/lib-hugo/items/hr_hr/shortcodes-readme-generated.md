---
| Shortcode | Opis |
| --- | --- |
| `fastcomments` | Ugniježđeni komentari s odgovorima, glasovanjem i @spominjanjima |
| `fastcomments-comment-count` | Broj komentara za jednu stranicu |
| `fastcomments-comment-count-bulk` | Broj komentara za više stranica u jednom zahtjevu (pogledajte [Masovni brojači komentara](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Widget za chat uživo |
| `fastcomments-collab-chat` | Kolaborativno inline komentiranje (zahtijeva `target`) |
| `fastcomments-image-chat` | Komentari za označavanje slika (zahtijeva `target`) |
| `fastcomments-recent-comments` | Nedavni komentari na cijeloj web-lokaciji |
| `fastcomments-recent-discussions` | Nedavno aktivne teme rasprava |
| `fastcomments-reviews-summary` | Sažetak recenzija s ocjenama u zvjezdicama |
| `fastcomments-top-pages` | Stranice o kojima se najviše raspravlja |
| `fastcomments-user-activity-feed` | Feed aktivnosti po korisniku (zahtijeva `userId`) |

### Primjeri

Broj komentara unutar teksta:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Chat uživo:

```text
\{{< fastcomments-live-chat >}}
```

Kolaborativni chat, ciljanje elementa sadržaja pomoću CSS selektora:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Chat za slike, ciljanje elementa slike pomoću CSS selektora:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Sažetak recenzija:

```text
\{{< fastcomments-reviews-summary >}}
```

Feed aktivnosti korisnika:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```
---