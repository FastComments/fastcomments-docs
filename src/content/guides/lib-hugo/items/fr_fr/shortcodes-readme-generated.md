| Shortcode | Description |
| --- | --- |
| `fastcomments` | Commentaires en fil de discussion avec réponses, votes et mentions @ |
| `fastcomments-comment-count` | Nombre de commentaires pour une seule page |
| `fastcomments-comment-count-bulk` | Nombres de commentaires pour plusieurs pages en une seule requête (voir [Comptage massif des commentaires](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Widget de chat en direct |
| `fastcomments-collab-chat` | Commentaires collaboratifs en ligne (requiert `target`) |
| `fastcomments-image-chat` | Commentaires d'annotation d'image (requiert `target`) |
| `fastcomments-recent-comments` | Commentaires récents sur le site |
| `fastcomments-recent-discussions` | Fils de discussion récemment actifs |
| `fastcomments-reviews-summary` | Résumé des évaluations par étoiles |
| `fastcomments-top-pages` | Pages les plus discutées |
| `fastcomments-user-activity-feed` | Flux d'activité par utilisateur (requiert `userId`) |

### Exemples

Nombre de commentaires intégré au texte :

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Chat en direct :

```text
\{{< fastcomments-live-chat >}}
```

Chat collaboratif, ciblant un élément de contenu par sélecteur CSS :

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Chat sur image, ciblant un élément image par sélecteur CSS :

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Résumé des avis :

```text
\{{< fastcomments-reviews-summary >}}
```

Flux d'activité utilisateur :

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```