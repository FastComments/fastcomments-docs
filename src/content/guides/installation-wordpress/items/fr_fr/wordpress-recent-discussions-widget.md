Le widget Recent Discussions affiche les pages de votre site ayant l'activité de commentaires la plus récente. Il est utile pour mettre en avant les fils de discussion qui continuent de recevoir des contributions, afin que les visiteurs puissent reprendre des conversations actives plutôt que d'atterrir sur des pages calmes.

## Options

- **Title** (optionnel) : L'en-tête affiché au-dessus de la liste. Par défaut : "Recent Discussions".
- **Count** (optionnel) : Nombre de discussions à afficher. Plage de 1 à 50. Par défaut : 20.

## How to Add It

### Inside a Post or Page

Dans l'éditeur de blocs, ajoutez un bloc **Shortcode** et collez :

[inline-code-attrs-start title = 'Shortcode du widget Recent Discussions'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

L'attribut `count` accepte toute valeur comprise entre 1 et 50.

### In a Sidebar or Footer (Classic Themes)

Allez dans **Apparence > Widgets** dans l'administration WordPress. Depuis l'inserteur de blocs, recherchez "FastComments" et choisissez **FastComments: Recent Discussions**. Faites-le glisser dans une barre latérale, l'en-tête ou une zone de pied de page, puis configurez le titre et le nombre depuis le panneau du widget.

### In a Block Theme (Full Site Editing)

Ouvrez l'**Éditeur de site** sous **Apparence > Éditeur**. Naviguez jusqu'à la partie de modèle où le widget doit apparaître, insérez un bloc **Legacy Widget**, et sélectionnez **FastComments: Recent Discussions** dans le menu déroulant.

## Troubleshooting

Le widget ne s'affiche que lorsque la configuration de FastComments est terminée et qu'un tenant ID est enregistré. Si la zone du widget est vide, complétez la configuration sous **FastComments** dans l'administration WordPress et rechargez la page.

Si l'ordre des discussions semble obsolète, vérifiez que les pages sous-jacentes ont fini de se synchroniser dans le tableau de bord FastComments. Le widget lit les données en direct, donc les commentaires fraîchement importés peuvent mettre un moment à apparaître.