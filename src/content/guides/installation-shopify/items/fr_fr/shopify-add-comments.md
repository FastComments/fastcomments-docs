Le bloc **FastComments** est le widget de commentaires principal. Ajoutez-le aux modèles d'articles de blog, aux modèles de produits, ou à toute autre page où vous souhaitez un fil de discussion ou un chat en direct.

### Ajouter le bloc

1. Ouvrez l'éditeur de thème Shopify (**Boutique en ligne > Thèmes > Personnaliser**).
2. Choisissez le modèle sur lequel vous souhaitez des commentaires : **Article de blog**, **Produit**, ou tout autre modèle de page ou de section.
3. Dans la section où vous souhaitez que les commentaires apparaissent, cliquez sur **Ajouter un bloc**.
4. Sous **Applications**, sélectionnez **FastComments**.
5. Cliquez sur **Enregistrer**.

Le bloc apparaît immédiatement. Il n'y a pas de Tenant ID à saisir ; le tenant de votre boutique est configuré automatiquement lors de l'installation de l'application.

### Paramètres

| Paramètre | Fonction | Valeur par défaut |
|---|---|---|
| Tenant ID (facultatif) | Remplace le tenant FastComments utilisé par le bloc. Laissez vide pour utiliser le tenant configuré automatiquement pour la boutique. Vous pouvez trouver un Tenant ID manuel sur fastcomments.com/auth/my-account/api-secret. | (vide) |
| SSO | Connecte automatiquement le visiteur avec son compte client Shopify avant de commenter. Voir [Connexion automatique des clients Shopify](/guide-installation-shopify.html#shopify-sso). | Activé |
| Style de commentaires | **Threaded** pour des réponses imbriquées et des votes, ou **Streaming** pour un fil de chat en temps réel. | Threaded |
| Custom URL ID | Remplace l'identifiant de page détecté automatiquement. Utilisez-le lorsque vous souhaitez que deux URL partagent un même fil de commentaires. | (détection automatique) |

### Comment l'identifiant de page est choisi

Chaque fil de commentaires est associé à un URL ID. Le bloc en choisit un automatiquement :

- **Modèle d'article de blog :** `shopify-article-{article.id}`, qui reste stable malgré les changements de slug et de titre.
- **Modèle de produit :** `shopify-product-{product.id}`, qui reste stable malgré les changements de slug et de titre.
- **Autres modèles :** le chemin de la requête.

Si vous définissez un **Custom URL ID**, cette valeur est utilisée à la place. Utilisez le même Custom URL ID sur plusieurs blocs (par exemple, sur une variante localisée d'une page produit) pour partager un même fil de commentaires.

### Threaded vs Streaming

**Threaded** est le mode par défaut. Les visiteurs se répondent, votent, et les outils de modération fonctionnent normalement. Idéal pour les articles de blog et les avis produits.

**Streaming** supprime l'imbrication et affiche les nouveaux commentaires en temps réel dès leur publication, comme un fil de chat. Idéal pour les lancements de produits, les événements en direct et les pages communautaires.

### Plusieurs blocs sur la même page

Le bloc peut être ajouté plusieurs fois au même modèle. Par exemple, un Résumé des avis en haut d'une page produit et un bloc FastComments en bas. Les blocs partagent un URL ID, de sorte que le résumé reflète les commentaires situés en dessous.

### Conseils

- Le bloc se masque dans l'aperçu de l'éditeur de thème avec une notice jaune s'il ne trouve pas de tenant. Si cela apparaît dans votre boutique en ligne, réinstallez l'application FastComments.
- Pour une page produit, le bloc FastComments fait également office de widget d'avis produits. Associez-le à **FastComments - Reviews Summary** pour afficher un résumé avec notation par étoiles en haut de la page.