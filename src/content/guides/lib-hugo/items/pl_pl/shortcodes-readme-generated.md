| Shortcode | Opis |
| --- | --- |
| `fastcomments` | Komentarze w wątkach z odpowiedziami, głosowaniem i @wzmiankami |
| `fastcomments-comment-count` | Liczba komentarzy dla pojedynczej strony |
| `fastcomments-comment-count-bulk` | Liczby komentarzy dla wielu stron w jednym żądaniu (zobacz [Zbiorcze liczniki komentarzy](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Widżet czatu na żywo |
| `fastcomments-collab-chat` | Współdzielone komentarze inline (wymaga `target`) |
| `fastcomments-image-chat` | Komentarze z adnotacjami obrazów (wymaga `target`) |
| `fastcomments-recent-comments` | Najnowsze komentarze w serwisie |
| `fastcomments-recent-discussions` | Ostatnio aktywne wątki dyskusyjne |
| `fastcomments-reviews-summary` | Podsumowanie recenzji z ocenami gwiazdkowymi |
| `fastcomments-top-pages` | Najczęściej dyskutowane strony |
| `fastcomments-user-activity-feed` | Kanał aktywności użytkownika (wymaga `userId`) |

### Przykłady

Liczba komentarzy w tekście:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Czat na żywo:

```text
\{{< fastcomments-live-chat >}}
```

Czat współpracy, kierowany do elementu treści za pomocą selektora CSS:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Czat obrazkowy, kierowany do elementu obrazu za pomocą selektora CSS:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Podsumowanie recenzji:

```text
\{{< fastcomments-reviews-summary >}}
```

Kanał aktywności użytkownika:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```