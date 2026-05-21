---
Si vous ne pouvez pas installer l'application [Shopify App Store app](https://apps.shopify.com/fastcomments), vous pouvez quand même ajouter FastComments en modifiant votre thème. Cette méthode est utile lorsque vous souhaitez connecter un tenant FastComments que vous possédez déjà, ou lorsque vous intégrez sur une vitrine Shopify où l'application n'est pas une option.

L'installation via l'application est la méthode prise en charge pour la plupart des boutiques. N'utilisez cette méthode que si l'application ne convient pas.

### Étape 1 : Désactiver les commentaires natifs de Shopify

Dans votre admin Shopify, allez à **Articles de blogue > Gérer les blogues**, ouvrez chaque blogue et définissez **Commentaires désactivés** dans le panneau de droite. Enregistrez.

Cela empêche le système de commentaires intégré de Shopify d'apparaître en même temps que FastComments.

### Étape 2 : Ouvrir le modèle de thème du blogue

Dans votre admin Shopify:

1. Allez à **Boutique en ligne > Thèmes**.
2. Sous votre thème actuel, cliquez **Actions > Modifier le code**.
3. Dans l'explorateur de fichiers à gauche, ouvrez **Sections** et cliquez sur `main-article.liquid`.

Ceci est le modèle que Shopify utilise pour afficher un article de blogue unique.

### Étape 3 : Coller l'extrait FastComments

Faites défiler jusqu'à environ la ligne 100 de `main-article.liquid`, juste après la balise de fermeture `</div>` du corps de l'article. Collez l'extrait suivant :

[inline-code-attrs-start title = 'Extrait FastComments pour Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

Remplacez `"demo"` par votre propre Tenant ID provenant de [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Cliquez sur **Enregistrer**.

### Étape 4 : Autoriser le domaine de votre boutique

Ouvrez un article de blogue sur votre boutique en ligne. Si vous voyez une erreur d'autorisation au lieu du widget de commentaires, FastComments doit savoir que votre boutique est autorisée à utiliser ce tenant. Consultez [Erreurs de domaine](/guide-installation-shopify.html#shopify-domain-errors).

### Ajouter FastComments à d'autres pages

Le même extrait fonctionne dans n'importe quel modèle Liquid, y compris les pages de produit, les pages personnalisées et la page d'accueil. Collez-le à l'endroit où vous voulez que les commentaires apparaissent et ajustez `urlId` si vous voulez un identifiant stable par page (par exemple, `urlId: "{{ product.id }}"` dans un modèle de produit).

---