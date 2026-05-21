Le bloc **FastComments** est le principal widget de commentaires. Ajoutez-le aux modèles d'articles de blog, aux modèles de produits, ou à toute autre page où vous souhaitez un fil de discussion ou un chat en direct.

### Ajouter le bloc

1. Ouvrez l'éditeur de thème Shopify (**Online Store > Themes > Customize**).
2. Choisissez le modèle sur lequel vous voulez des commentaires : **Blog post**, **Product**, ou tout autre modèle de page ou de section.
3. Dans la section où vous souhaitez que les commentaires apparaissent, cliquez sur **Add block**.
4. Sous **Apps**, sélectionnez **FastComments**.
5. Cliquez sur **Save**.

Le bloc apparaît immédiatement. Il n'y a pas de Tenant ID à saisir ; le tenant de votre boutique est configuré automatiquement lors de l'installation de l'application.

### Paramètres

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Remplace le tenant FastComments contre lequel le bloc est rendu. Laissez vide pour utiliser le tenant automatiquement configuré de la boutique. Trouvez un Tenant ID manuel sur fastcomments.com/auth/my-account/api-secret. | (blank) |
| SSO | Connecte automatiquement le visiteur en tant que client Shopify avant de commenter. Voir [Auto-Login Shopify Customers](/guide-installation-shopify.html#shopify-sso). | On |
| Commenting Style | **Threaded** pour des réponses imbriquées et des votes, ou **Streaming** pour un flux de chat en temps réel. | Threaded |
| Custom URL ID | Remplace l'identifiant de page détecté automatiquement. Utilisez ceci quand vous voulez que deux URL partagent un même fil de commentaires. | (auto-detected) |

### Comment l'identifiant de page est choisi

Chaque fil de commentaires est indexé par un URL ID. Le bloc en sélectionne un automatiquement :

- **Blog post template:** `shopify-article-{article.id}`, which is stable across slug and title changes.
- **Product template:** `shopify-product-{product.id}`, which is stable across slug and title changes.
- **Other templates:** the request path.

Si vous définissez **Custom URL ID**, cette valeur est utilisée à la place. Utilisez le même Custom URL ID sur plusieurs blocs (par exemple, sur une variante localisée d'une page produit) pour partager un même fil de commentaires.

### Threaded vs Streaming

**Threaded** est le mode par défaut. Les visiteurs se répondent entre eux, votent, et les outils de modération fonctionnent comme prévu. Idéal pour les articles de blog et les avis produits.

**Streaming** supprime l'imbrication et affiche les nouveaux commentaires en temps réel au fur et à mesure de leur publication, comme un flux de chat. Idéal pour les lancements de produits, les événements en direct et les pages communautaires.

### Plusieurs blocs sur la même page

Le bloc peut être ajouté plusieurs fois au même modèle. Par exemple, un résumé des avis en haut d'une page produit et un bloc FastComments en bas. Les blocs partagent un URL ID, donc le résumé reflète les commentaires ci-dessous.

### Astuces

- Le bloc se cache dans l'aperçu de l'éditeur de thème avec un avis jaune s'il ne trouve pas de tenant. Si cela apparaît dans votre boutique en production, réinstallez l'application FastComments.
- Pour une page produit, le bloc FastComments fait aussi office de widget d'avis produit. Associez-le à **FastComments - Reviews Summary** pour un résumé par étoiles en haut de la page.