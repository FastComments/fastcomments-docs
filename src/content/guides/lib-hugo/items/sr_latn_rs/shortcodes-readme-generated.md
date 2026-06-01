| Shortcode | Opis |
| --- | --- |
| `fastcomments` | Ugnježdeni komentari sa odgovorima, glasovanjem i @mentions |
| `fastcomments-comment-count` | Broj komentara za jednu stranicu |
| `fastcomments-comment-count-bulk` | Brojevi komentara za više stranica u jednom zahtevu (pogledajte [Grupni brojevi komentara](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Widget za chat uživo |
| `fastcomments-collab-chat` | Kolaborativno komentarisanje u liniji (zahteva `target`) |
| `fastcomments-image-chat` | Komentari za anotaciju slike (zahteva `target`) |
| `fastcomments-recent-comments` | Nedavni komentari na sajtu |
| `fastcomments-recent-discussions` | Nedavno aktivne diskusije |
| `fastcomments-reviews-summary` | Sažetak recenzija sa zvezdicama |
| `fastcomments-top-pages` | Najdiskutovanije stranice |
| `fastcomments-user-activity-feed` | Feed aktivnosti po korisniku (zahteva `userId`) |

### Primeri

Broj komentara umetnut u tekst:

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