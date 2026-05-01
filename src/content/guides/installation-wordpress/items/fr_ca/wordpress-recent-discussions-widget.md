Le widget Discussions récentes affiche les pages de votre site présentant l'activité de commentaires la plus récente. Il est utile pour mettre en avant les fils de discussion qui continuent d'être alimentés, afin que les visiteurs puissent reprendre des conversations actives plutôt que d'arriver sur des pages calmes.

## Options

- **Titre** (optionnel) : Le titre affiché au‑dessus de la liste. Par défaut : "Discussions récentes".
- **Nombre** (optionnel) : Le nombre de discussions à afficher. Plage de 1 à 50. Par défaut : 20.

## Comment l'ajouter

### Dans un article ou une page

Dans l'éditeur de blocs, ajoutez un bloc **Shortcode** et collez :

[inline-code-attrs-start title = 'Shortcode Discussions récentes'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

L'attribut `count` accepte n'importe quelle valeur entre 1 et 50.

### Dans une barre latérale ou le pied de page (thèmes classiques)

Allez dans **Apparence > Widgets** dans l'administration WordPress. Dans l'inséreur de blocs, cherchez "FastComments" et choisissez **FastComments: Discussions récentes**. Faites-le glisser dans une barre latérale, l'en-tête ou le pied de page, puis configurez le titre et le nombre depuis le panneau du widget.

### Dans un thème de blocs (édition complète du site)

Ouvrez l'**Éditeur du site** sous **Apparence > Éditeur**. Accédez à la partie du modèle où le widget doit apparaître, insérez un bloc **Legacy Widget**, puis sélectionnez **FastComments: Discussions récentes** dans le menu déroulant.

## Dépannage

Le widget ne s'affiche qu'une fois la configuration de FastComments terminée et qu'un tenant ID a été enregistré. Si la zone du widget est vide, complétez la configuration sous **FastComments** dans l'administration WordPress et rechargez la page.

Si l'ordre des discussions semble obsolète, vérifiez que les pages sous-jacentes ont terminé leur synchronisation dans le tableau de bord FastComments. Le widget lit des données en direct, donc les commentaires fraîchement importés peuvent mettre un moment à apparaître.