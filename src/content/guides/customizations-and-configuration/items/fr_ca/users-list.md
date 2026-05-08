[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments n'affiche pas de liste d'utilisateurs sur la page.

Vous pouvez afficher une liste des personnes qui consultent actuellement la page, à côté du widget de commentaires. La liste se met à jour en temps réel lorsque des utilisateurs arrivent ou partent, et montre leur nom, leur avatar et un indicateur de présence en ligne.

Il y a trois options de mise en page :

- `1` - Top: a horizontal row of overlapping avatars rendered above the comments.
- `2` - Left: a sidebar with names and online dots rendered to the left of the widget.
- `3` - Right: the same sidebar rendered to the right of the widget.

Définissez le paramètre **usersListLocation** pour activer la fonctionnalité :

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Par défaut, la liste n'affiche que les utilisateurs actuellement en ligne. Pour inclure également les personnes qui ont commenté la page dans le passé (mais qui ne la consultent pas actuellement), définissez **usersListIncludeOffline** sur true :

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Les commentateurs passés sont affichés sans la pastille verte indiquant la présence en ligne, de sorte qu'il est clair qui est présent en ce moment.

Les utilisateurs ayant des profils privés sont affichés avec un avatar générique et une étiquette "Private Profile" afin que le nombre reste exact sans révéler les identités.

Cela peut aussi être configuré sans code. Sur la page de personnalisation du widget, consultez l'option "Users List Location" :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Lorsque l'emplacement est défini sur autre chose que Off, la case à cocher "Include past commenters" est affichée en dessous :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]

---