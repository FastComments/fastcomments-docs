[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments n'affiche pas de liste d'utilisateurs sur la page.

Vous pouvez afficher une liste des personnes qui consultent actuellement la page, à côté du widget de commentaires. La liste se met à jour en direct lorsque des utilisateurs arrivent et partent, et affiche leur nom, avatar et un indicateur en ligne.

Il y a trois options de disposition :

- `1` - Top : une rangée horizontale d'avatars qui se chevauchent affichée au-dessus des commentaires.
- `2` - Left : une barre latérale avec des noms et des points en ligne affichée à gauche du widget.
- `3` - Right : la même barre latérale affichée à droite du widget.

Définissez le paramètre **usersListLocation** pour activer la fonctionnalité :

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Par défaut, la liste n'affiche que les utilisateurs actuellement en ligne. Pour inclure également les personnes qui ont commenté la page par le passé (mais qui ne la consultent pas actuellement), définissez **usersListIncludeOffline** sur true :

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Les commentateurs précédents sont affichés sans le point vert en ligne, afin qu'il soit clair qui est présent en ce moment.

Les utilisateurs ayant un profil privé sont affichés avec un avatar générique et une étiquette "Profil privé" afin que le décompte reste précis sans révéler les identités.

Cela peut aussi être configuré sans code. À la page de personnalisation du widget, consultez l'option "Emplacement de la liste d'utilisateurs". Lorsque l'emplacement est défini sur autre chose que "Désactivé", une case à cocher "Inclure les commentateurs précédents" apparaît en dessous.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Au-delà de 500 utilisateurs en direct, la liste peut avoir un retard allant jusqu'à 30 secondes.

---