[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Par défaut, les commentaires en direct sont activés dans FastComments.

Cela signifie que tous les visiteurs du fil de commentaires verront le même contenu.

Par exemple, si un commentaire est ajouté, ce commentaire s'affichera. Si un commentaire est modifié ou supprimé,
alors ces commentaires seront modifiés ou supprimés pour tous les visiteurs du fil. Il en va de même pour les votes et toutes les actions de modération.

Cependant, nous pouvons désactiver cela :

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Ceci peut aussi être fait sans code. Dans la page de personnalisation du widget, consultez la section "Désactiver les commentaires en direct".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]