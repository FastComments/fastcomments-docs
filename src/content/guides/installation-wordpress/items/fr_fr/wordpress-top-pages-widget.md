Le widget Top Pages affiche les pages de votre site ayant le plus de commentaires. Il est utile pour mettre en avant vos contenus les plus engageants auprès des nouveaux visiteurs et augmenter le temps passé sur le site.

## Options

- **Title** (optionnel) : Le titre affiché au-dessus de la liste. Par défaut : "Top Pages".

Le widget Top Pages choisit sa propre mise en page en fonction des données disponibles et n'accepte pas d'attribut count.

## Comment l'ajouter

### Dans un article ou une page

Dans l'éditeur de blocs, ajoutez un bloc **Shortcode** et collez :

[inline-code-attrs-start title = 'Shortcode Top Pages'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### Dans une barre latérale ou le pied de page (thèmes classiques)

Allez dans **Apparence > Widgets** dans l'administration WordPress. Dans l'insertion de blocs, cherchez "FastComments" et choisissez **FastComments: Top Pages**. Faites-le glisser dans une barre latérale, une zone d'en-tête ou de pied de page, puis définissez le titre depuis le panneau du widget.

### Dans un thème par blocs (édition complète du site)

Ouvrez l'**Éditeur du site** sous **Apparence > Éditeur**. Allez à la partie de modèle où le widget doit apparaître, insérez un bloc **Legacy Widget**, puis sélectionnez **FastComments: Top Pages** dans le menu déroulant.

## Dépannage

Le widget ne s'affiche que lorsque la configuration de FastComments est terminée et qu'un tenant ID est enregistré. Si la zone du widget est vide, terminez la configuration sous **FastComments** dans l'administration WordPress et rechargez la page.