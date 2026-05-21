Le bloc **FastComments - Comment Count** affiche un petit compteur de commentaires pour une seule page. Utilisez-le dans les listes d'articles de blog, les fiches produit, ou tout modèle qui renvoie à une page avec des commentaires, afin que les visiteurs puissent voir l'activité de chaque fil avant de cliquer.

### Add the block

1. Ouvrez l'éditeur de thème Shopify.
2. Ouvrez le template où vous souhaitez que le compteur apparaisse. Par exemple, le **Blog** template (la liste d'articles) ou une section de listing produit.
3. Cliquez sur **Add block** dans la section qui affiche chaque élément.
4. Sous **Apps**, sélectionnez **FastComments - Comment Count**.
5. Cliquez sur **Save**.

### Settings

| Paramètre | Ce que ça fait | Par défaut |
|---|---|---|
| Tenant ID (optional) | Surcharge le tenant FastComments à partir duquel le compteur lit les données. Laissez vide pour utiliser le tenant automatiquement configuré pour la boutique. | (vide) |
| Custom URL ID | Surcharge l'identifiant de page que le compteur recherche. Utilisez ceci lorsque le compteur se trouve sur une page différente du bloc FastComments qu'il suit. | (détecté automatiquement) |

### How the count matches the comment thread

Le bloc Comment Count utilise la même logique d'auto-détection que le bloc **FastComments** :

- Blog post template: `shopify-article-{article.id}`
- Product template: `shopify-product-{product.id}`
- Other templates: the request path

Si vous définissez un **Custom URL ID** sur le bloc **FastComments** d'une page, définissez le même Custom URL ID sur le bloc Comment Count afin qu'ils pointent vers le même fil.

### Tips

- Les compteurs pour tous les éléments de la page sont récupérés en une seule requête, donc ajouter le bloc à chaque élément d'une longue liste n'entraîne pas de coût supplémentaire en allers-retours.
- Un bloc Comment Count par article ou produit dans une liste est l'utilisation attendue ; le bloc peut être ajouté autant de fois que nécessaire.