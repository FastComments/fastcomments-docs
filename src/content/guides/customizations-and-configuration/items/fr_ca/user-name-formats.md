Par défaut, FastComments affichera le nom de l'utilisateur tel qu'il l'a saisi, ou tel qu'il nous a été transmis via SSO.

Cependant, il peut être souhaitable de masquer ou d'afficher le nom de l'utilisateur d'une autre façon. Par exemple, si le nom de l'utilisateur est Allen Rex, vous pourriez
vouloir n'afficher que "Allen R.".

Cela peut être fait sans code dans l'interface de personnalisation du widget, sous le réglage appelé `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

Les formats disponibles sont :

- Capitaliser (affiche Example User comme Example User)
- Initiale du nom (affiche Example User comme Example U.)
- Toutes les initiales (affiche Example User comme E. U.)
- Afficher "Anonyme"

L'effet de ce changement est immédiat. Les utilisateurs verront toujours leur nom d'utilisateur complet en haut de la zone de commentaire, pour eux-mêmes, mais leurs commentaires afficheront
le nom d'utilisateur modifié.

Les noms d'utilisateur sont masqués côté serveur pour protéger les utilisateurs.