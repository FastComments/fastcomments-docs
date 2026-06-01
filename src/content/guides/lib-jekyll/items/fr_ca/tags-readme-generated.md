---
| Balise | Description |
| --- | --- |
| `fastcomments` | Commentaires en direct avec réponses, votes, modération et mises à jour en temps réel |
| `fastcomments_comment_count` | Nombre de commentaires pour la page courante |
| `fastcomments_comment_count_bulk` | Nombre de commentaires pour plusieurs pages sur une page de liste/index |
| `fastcomments_live_chat` | Widget de chat en streaming en temps réel |
| `fastcomments_collab_chat` | Commentaires collaboratifs en ligne (annotations de texte) |
| `fastcomments_image_chat` | Commentaires d'annotation d'image |
| `fastcomments_recent_comments` | Commentaires récents sur l'ensemble du site |
| `fastcomments_recent_discussions` | Fils de discussion récemment actifs |
| `fastcomments_reviews_summary` | Résumé des avis par étoiles |
| `fastcomments_top_pages` | Pages les plus discutées |
| `fastcomments_user_activity_feed` | Fil d'activité par utilisateur |

### Exemples

```liquid
{% raw %}{# Nombre de commentaires. Le widget affiche son propre libellé, p. ex. "0 commentaires" #}
{% fastcomments_comment_count %}

{# Chat en direct #}
{% fastcomments_live_chat %}

{# Chat collaboratif. Pointez-le vers un élément de contenu avec un sélecteur CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Chat d'image. Pointez-le vers un élément image avec un sélecteur CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Résumé des avis #}
{% fastcomments_reviews_summary %}

{# Fil d'activité utilisateur. Nécessite un ID utilisateur #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Comptes de commentaires en bloc pour un index de blogue #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```