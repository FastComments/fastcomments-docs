Le widget Discussions récentes affiche les pages de votre site ayant l'activité de commentaires la plus récente. Il est utile pour mettre en avant les discussions encore actives, afin que les visiteurs puissent revenir dans des conversations en cours plutôt que d'atterrir sur des pages silencieuses.

## Options

- **Titre** (facultatif) : le titre affiché au-dessus de la liste. Par défaut : « Recent Discussions ».
- **Nombre** (facultatif) : le nombre de discussions à afficher. Entre 1 et 50. Par défaut : 20.

## Comment l'ajouter

### Dans un article ou une page

Dans l'éditeur de blocs, ajoutez un bloc **Shortcode** et collez :

[inline-code-attrs-start title = 'Shortcode Discussions récentes'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

L'attribut `count` accepte toute valeur entre 1 et 50.

### Dans une barre latérale ou un pied de page (thèmes classiques)

Allez dans **Apparence > Widgets** dans l'administration WordPress. Depuis l'inserteur de blocs, recherchez « FastComments » et choisissez **FastComments: Recent Discussions**. Faites-le glisser dans une zone de barre latérale, d'en-tête ou de pied de page, puis configurez le titre et le nombre depuis le panneau du widget.

### Dans un thème en blocs (édition complète du site)

Ouvrez l'**Éditeur de site** sous **Apparence > Éditeur**. Accédez à la partie de modèle où le widget doit apparaître, insérez un bloc **Legacy Widget** et sélectionnez **FastComments: Recent Discussions** dans la liste déroulante.

## Dépannage

Le widget ne s'affiche qu'une fois la configuration de FastComments terminée et un identifiant de tenant enregistré. Si la zone du widget est vide, terminez la configuration sous **FastComments** dans l'administration WordPress et rechargez la page.

Si l'ordre des discussions semble obsolète, vérifiez que les pages sous-jacentes ont terminé leur synchronisation dans le tableau de bord FastComments. Le widget lit les données en direct, les commentaires récemment importés peuvent donc prendre un moment avant d'apparaître.