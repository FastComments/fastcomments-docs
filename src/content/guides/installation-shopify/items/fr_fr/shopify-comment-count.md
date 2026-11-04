Le bloc **FastComments - Comment Count** affiche un petit nombre de commentaires pour une seule page. Utilisez-le dans les listes d'articles de blog, les fiches produit, ou tout modèle qui renvoie vers une page avec des commentaires, afin que les visiteurs puissent voir l'activité de chaque discussion avant de cliquer.

### Ajouter le bloc

1. Ouvrez l'éditeur de thème Shopify.
2. Ouvrez le modèle où vous souhaitez que le compteur apparaisse. Par exemple, le modèle **Blog** (la liste d'articles) ou une section de liste de produits.
3. Cliquez sur **Ajouter un bloc** dans la section qui affiche chaque élément.
4. Sous **Applications**, sélectionnez **FastComments - Comment Count**.
5. Cliquez sur **Enregistrer**.

### Paramètres

| Paramètre | Fonction | Valeur par défaut |
|---|---|---|
| Tenant ID (facultatif) | Remplace le tenant FastComments à partir duquel le compteur lit les données. Laissez vide pour utiliser le tenant configuré automatiquement pour la boutique. | (vide) |
| Custom URL ID | Remplace l'identifiant de page recherché par le compteur. Utilisez-le lorsque le compteur se trouve sur une page différente du bloc FastComments qu'il suit. | (détection automatique) |

### Correspondance entre le compteur et le fil de commentaires

Le bloc Comment Count utilise la même logique de détection automatique que le bloc **FastComments** :

- Modèle d'article de blog : `shopify-article-{article.id}`
- Modèle de produit : `shopify-product-{product.id}`
- Autres modèles : le chemin de la requête

Si vous définissez un **Custom URL ID** sur le bloc **FastComments** d'une page, définissez le même Custom URL ID sur le bloc Comment Count afin qu'ils pointent vers le même fil de discussion.

### Conseils

- Les compteurs de tous les éléments de la page sont récupérés en une seule requête, donc ajouter le bloc à chaque élément d'une longue liste n'entraîne aucun coût supplémentaire d'aller-retour.
- Un bloc Comment Count par article ou produit dans une liste est l'utilisation prévue ; le bloc peut être ajouté autant de fois que nécessaire.