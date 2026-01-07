Le Widget de Comptage de Commentaires en Masse est concu pour afficher efficacement le nombre de commentaires pour plusieurs pages sur la meme page. Au lieu de faire des appels API individuels pour chaque comptage de commentaires, ce widget regroupe les requetes pour une performance optimale.

## Installation de Base

[inline-code-attrs-start title = 'Bulk Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Comment ca Fonctionne

Le widget en masse fonctionne en:

1. Scannant la page pour les elements avec la classe `fast-comments-count`
2. Lisant l'attribut `data-fast-comments-url-id` de chaque element
3. Regroupant les requetes API pour recuperer efficacement plusieurs comptages de commentaires
4. Mettant a jour chaque element avec le comptage de commentaires approprie

## Options de Configuration

La fonction `FastCommentsCommentCountBulk` accepte les options de configuration suivantes:

- **tenantId** (requis): Votre ID de tenant FastComments
- **apiHost** (optionnel): Hote API personnalise si vous utilisez une instance auto-hebergee

## Exemple Concret

Voici un exemple pratique montrant comment vous pourriez utiliser le widget en masse dans une liste d'articles de blog:

[inline-code-attrs-start title = 'Blog Post Listing with Comment Counts'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Considerations de Performance

Le widget en masse optimise automatiquement la performance par:

- **Regroupement des requetes**: Plusieurs comptages de commentaires sont recuperes en un seul appel API
- **Limites de taille des requetes**: Les requetes sont automatiquement divisees si la liste d'URLs devient trop grande (plus de 1 000 caracteres)
- **Deduplication**: Plusieurs elements avec le meme `data-fast-comments-url-id` partagent le meme comptage

## Plusieurs Elements avec le Meme URL ID

Vous pouvez avoir plusieurs elements sur la page avec le meme `data-fast-comments-url-id`. Ils seront tous mis a jour avec le meme comptage:

[inline-code-attrs-start title = 'Multiple Elements Same URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Localisation

Le widget en masse formate automatiquement les comptages de commentaires selon vos parametres de langue FastComments. Il fournit un texte approprie pour:

- Zero commentaires
- Un commentaire
- Plusieurs commentaires

## Quand Utiliser le Widget en Masse vs le Widget Individuel

**Utilisez le Widget en Masse quand:**
- Vous avez plusieurs comptages de commentaires sur la meme page
- Vous affichez une liste d'articles/publications avec des comptages de commentaires
- La performance est importante (reduit les appels API)

**Utilisez le Widget Individuel quand:**
- Vous n'avez besoin que d'un seul comptage de commentaires sur la page
- Vous avez besoin de mises a jour en direct (le widget individuel prend en charge les mises a jour en temps reel)
- Vous voulez plus de controle sur le comportement individuel du widget
