Le bloc **FastComments - Résumé des avis** affiche une note étoilée agrégée et une répartition des avis pour une page. Associez-le au bloc **FastComments** sur les modèles de produit pour la mise en page standard des avis : résumé en haut, formulaire d'avis et avis en dessous.

### Prérequis : configurer Ratings & Reviews

Le bloc Résumé des avis affiche les questions d'évaluation que vous avez configurées pour votre boutique. Configurez-les d'abord :

1. Ouvrez l'application FastComments dans votre administration Shopify.
2. Cliquez sur la tuile **Ratings & Reviews Helper** (ou ouvrez directement [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify)).
3. Ajoutez les questions que vous souhaitez que chaque évaluateur réponde (note globale en étoiles, « comment était la taille », etc.).

Sans questions configurées, le bloc résumé n'a rien à agréger.

### Ajouter le bloc

1. Ouvrez l'éditeur de thème Shopify.
2. Ouvrez le modèle **Product** (ou le modèle de page où vous voulez le résumé).
3. Cliquez sur **Add block** près du haut de la section de la page, au-dessus de l'endroit où se trouvera le bloc **FastComments**.
4. Sous **Apps**, sélectionnez **FastComments - Reviews Summary**.
5. Ajoutez un bloc **FastComments** plus bas sur la même page si vous ne l'avez pas déjà fait, afin que les visiteurs puissent laisser des avis.
6. Cliquez sur **Save**.

### Paramètres

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Remplace le tenant FastComments dont le résumé lit les données. Laissez vide pour utiliser le tenant configuré automatiquement pour la boutique. | (vide) |
| Custom URL ID | Remplace l'identifiant de page contre lequel le résumé s'agrège. Utilisez ceci lorsque le résumé se trouve sur une page différente du bloc FastComments qu'il reflète. | (auto-detected) |

### Comment le résumé correspond aux avis

Le bloc Résumé des avis utilise la même logique de détection automatique que le bloc **FastComments** :

- Modèle produit : `shopify-product-{product.id}`
- Modèle d'article de blog : `shopify-article-{article.id}`
- Autres modèles : le chemin de la requête

Pour une page produit normale, le résumé et le fil de commentaires partagent automatiquement un ID d'URL, sans configuration nécessaire.

### Conseils

- Le résumé est en lecture seule. Pour collecter des avis, vous avez besoin d'un bloc **FastComments** sur la même page.
- Si vous modifiez les questions d'évaluation dans Ratings & Reviews Helper après avoir collecté des avis, le résumé est recalculé en fonction du nouvel ensemble de questions.