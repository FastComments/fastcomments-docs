[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments n'affiche pas de liste d'utilisateurs sur la page.

Vous pouvez afficher une liste de personnes qui consultent actuellement la page, à côté du widget de commentaires. La liste se met à jour en temps réel lorsque des utilisateurs arrivent ou partent, et affiche leur nom, avatar et un indicateur de présence.

Il existe trois options de mise en page :

- `1` - En haut : une rangée horizontale d'avatars qui se chevauchent affichée au-dessus des commentaires.
- `2` - Gauche : une barre latérale avec les noms et des pastilles indiquant la présence affichée à gauche du widget.
- `3` - Droite : la même barre latérale affichée à droite du widget.

Définissez le flag **usersListLocation** pour activer la fonctionnalité :

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Par défaut, la liste n'affiche que les utilisateurs actuellement en ligne. Pour inclure également les personnes qui ont commenté la page dans le passé (mais qui ne la consultent pas actuellement), réglez **usersListIncludeOffline** sur true :

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Les commentateurs passés sont affichés sans la pastille verte de présence afin qu'il soit clair qui est présent en ce moment.

Les utilisateurs ayant un profil privé sont affichés avec un avatar générique et un libellé "Profil privé" afin que le nombre reste exact sans révéler les identités.

Cela peut aussi être configuré sans code. Sur la page de personnalisation du widget, consultez l'option "Emplacement de la liste d'utilisateurs". Lorsque l'emplacement est réglé sur autre chose que Désactivé, une case à cocher "Inclure les commentateurs précédents" apparaît en dessous.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Au-delà de 500 utilisateurs en direct, la liste peut être obsolète jusqu'à 30 secondes.