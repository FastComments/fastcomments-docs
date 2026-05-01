Le widget Pages les plus commentées affiche les pages les plus commentées de votre site. Il est utile pour mettre en avant votre contenu le plus engageant auprès des nouveaux visiteurs et augmenter le temps passé sur le site.

## Options

- **Title** (optionnel) : Le titre affiché au-dessus de la liste. Par défaut : "Pages les plus commentées".

Le widget Pages les plus commentées choisit sa propre disposition en fonction des données disponibles et n'accepte pas d'attribut count.

## How to Add It

### Inside a Post or Page

Dans l'éditeur de blocs, ajoutez un **Shortcode** block et collez :

[inline-code-attrs-start title = 'Shortcode Pages les plus commentées'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### In a Sidebar or Footer (Classic Themes)

Allez dans **Apparence > Widgets** dans votre administration WordPress. Dans l'inserteur de blocs, recherchez "FastComments" et choisissez **FastComments : Pages les plus commentées**. Faites-le glisser dans une barre latérale, une zone d'en-tête ou de pied de page, puis définissez le titre depuis le panneau du widget.

### In a Block Theme (Full Site Editing)

Ouvrez l'**Éditeur du site** sous **Apparence > Éditeur**. Naviguez jusqu'à la partie de modèle où le widget doit apparaître, insérez un bloc **Legacy Widget**, et sélectionnez **FastComments : Pages les plus commentées** dans le menu déroulant.

## Troubleshooting

Le widget ne s'affiche qu'après la configuration de FastComments et qu'un tenant ID est enregistré. Si la zone du widget est vide, complétez la configuration sous **FastComments** dans l'administration WordPress et rechargez la page.