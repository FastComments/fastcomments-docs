[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments active les commentaires en direct.

Cela signifie que chaque visiteur du fil de commentaires devrait voir le même contenu.

Par exemple, si un commentaire est ajouté, ce commentaire devrait s'afficher. Si un commentaire est modifié ou supprimé,
alors ces commentaires seront modifiés ou supprimés pour tous les visiteurs du fil. Il en va de même pour les votes et toutes les actions de modération.

Cependant, nous pouvons désactiver cela :

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Cela peut également être fait sans code. Dans la page de personnalisation du widget, consultez la section "Désactiver les commentaires en direct".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]