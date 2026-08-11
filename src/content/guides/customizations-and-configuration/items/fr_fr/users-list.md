[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments n'affiche pas de liste d'utilisateurs sur la page.

Vous pouvez rendre une liste de personnes qui consultent actuellement la page, à côté du widget de commentaires. La liste se met à jour en temps réel lorsque des utilisateurs se joignent ou quittent, et affiche leur nom, avatar et un indicateur en ligne.

Il existe trois options de mise en page :

- `1` - Haut : une rangée horizontale d'avatars qui se chevauchent, affichée au-dessus des commentaires.
- `2` - Gauche : une barre latérale avec les noms et les points en ligne affichée à gauche du widget.
- `3` - Droite : la même barre latérale affichée à droite du widget.

Définissez le drapeau **usersListLocation** pour activer la fonctionnalité :

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Afficher la liste des utilisateurs à droite'; code-example-end]

Par défaut, la liste ne montre que les utilisateurs actuellement en ligne. Pour inclure également les personnes qui ont commenté la page par le passé (mais ne la consultent pas actuellement), définissez **usersListIncludeOffline** sur true :

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Inclure les commentateurs passés'; code-example-end]

Les commentateurs passés sont affichés sans le point vert en ligne afin qu'il soit clair qui est présent en ce moment.

Les utilisateurs avec des profils privés sont affichés avec un avatar générique et une étiquette « Profil privé » afin que le compte reste précis sans révéler les identités.

Cela peut également être configuré sans code. Dans la page de personnalisation du widget, voyez l'option « Users List Location ». Lorsque l'emplacement est réglé sur autre chose que Off, une case à cocher « Include past commenters » apparaît en dessous.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Emplacement de la liste des utilisateurs réglé sur la droite, avec la case à cocher « Inclure les anciens commentateurs » affichée en dessous'; title='Paramètres de la liste des utilisateurs'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Après 500 utilisateurs en direct, la liste peut être décalée de jusqu'à 30 secondes.