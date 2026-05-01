Le widget Recent Comments affiche les commentaires les plus récents publiés sur l’ensemble de votre site. Il est utile dans les barres latérales, les pieds de page ou partout où vous souhaitez mettre en avant une activité récente pour encourager la lecture.

## Options

- **Title** (optionnel) : Le titre affiché au-dessus de la liste. Par défaut : "Commentaires récents".
- **Count** (optionnel) : Nombre de commentaires à afficher. Plage de 1 à 50. Par défaut : 5.

## Comment l'ajouter

### Dans un article ou une page

Dans l'éditeur de blocs, ajoutez un bloc **Shortcode** et collez :

[inline-code-attrs-start title = 'Shortcode des commentaires récents'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

L'attribut `count` accepte une valeur entre 1 et 50.

### Dans une barre latérale ou un pied de page (thèmes classiques)

Allez dans **Apparence > Widgets** dans l'administration WordPress. Dans l'inserteur de blocs, cherchez "FastComments" et choisissez **FastComments: Recent Comments**. Faites-le glisser dans une barre latérale, un en-tête ou une zone de pied de page, puis configurez le titre et le nombre depuis le panneau du widget.

### Dans un thème basé sur des blocs (édition complète du site)

Ouvrez l'**Éditeur du site** sous **Apparence > Éditeur**. Naviguez jusqu'à la partie de modèle où le widget doit apparaître, insérez un bloc **Legacy Widget**, et sélectionnez **FastComments: Recent Comments** dans le menu déroulant.

## Dépannage

Le widget ne s'affiche que lorsque la configuration de FastComments est terminée et qu'un tenant ID est enregistré. Si la zone du widget est vide, terminez la configuration sous **FastComments** dans l'administration WordPress et rechargez la page.