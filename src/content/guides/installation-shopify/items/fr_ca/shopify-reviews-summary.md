Le bloc **FastComments - Résumé des avis** affiche une note globale (en étoiles) et une répartition des avis pour une page. Associez-le au bloc **FastComments** sur les modèles de produit pour la mise en page standard des avis : résumé en haut, formulaire d'avis et avis en dessous.

### Prérequis : configurer Évaluations et avis

Le bloc Résumé des avis affiche les questions d'évaluation que vous avez configurées pour votre boutique. Configurez-les d'abord :

1. Ouvrez l'application FastComments dans l'administration Shopify.
2. Cliquez sur la vignette **Assistant Évaluations et avis** (ou ouvrez directement [Assistant Évaluations et avis](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify)).
3. Ajoutez les questions auxquelles vous voulez que chaque évaluateur réponde (note globale en étoiles, "comment était la coupe", etc.).

Sans questions configurées, le bloc de résumé n'a rien à agréger.

### Ajouter le bloc

1. Ouvrez l'éditeur de thème Shopify.
2. Ouvrez le modèle **Product** (ou le modèle de page où vous voulez le résumé).
3. Cliquez sur **Add block** près du haut de la section de la page, au-dessus de l'endroit où se trouvera le bloc **FastComments**.
4. Sous **Apps**, sélectionnez **FastComments - Résumé des avis**.
5. Ajoutez un bloc **FastComments** plus bas sur la même page si ce n'est pas déjà fait, afin que les visiteurs puissent laisser des avis.
6. Cliquez sur **Save**.

### Paramètres

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Remplace le locataire FastComments dont le résumé lit les données. Laissez vide pour utiliser le locataire automatiquement configuré pour la boutique. | (vide) |
| Custom URL ID | Remplace l'identifiant de page contre lequel le résumé agrège les données. Utilisez ceci lorsque le résumé se trouve sur une page différente du bloc FastComments qu'il reflète. | (détecté automatiquement) |

### Comment le résumé correspond aux avis

Le bloc Résumé des avis utilise la même logique de détection automatique que le bloc **FastComments** :

- Product template: `shopify-product-{product.id}`
- Blog post template: `shopify-article-{article.id}`
- Other templates: the request path

Sur une page produit normale, le résumé et le fil de commentaires partagent automatiquement un ID d'URL, sans configuration requise.

### Conseils

- Le résumé est en lecture seule. Pour recueillir des avis, vous devez avoir un bloc **FastComments** sur la même page.
- Si vous modifiez les questions d'évaluation dans l'Assistant Évaluations et avis après avoir recueilli des avis, le résumé se recalcule en fonction du nouvel ensemble de questions.

---