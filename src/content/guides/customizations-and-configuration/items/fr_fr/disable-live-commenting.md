[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments aura les commentaires en direct activés.

Cela signifie que chaque lecteur du fil de commentaires devrait voir le même contenu.

Par exemple, si un commentaire est ajouté, ce commentaire doit s'afficher. Si un commentaire est modifié ou supprimé,
alors ces commentaires seront modifiés ou supprimés pour tous les lecteurs du fil. Il en va de même pour les votes et toutes les actions de modération.

Cependant, nous pouvons désactiver cela :

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Désactiver les commentaires en direct'; code-example-end]

Cela peut également être fait sans code. Dans la page de personnalisation du widget, consultez la section "Désactiver les commentaires en direct".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Section Désactiver les commentaires en direct de la page de personnalisation du widget, désactivant les mises à jour en temps réel du fil de discussion'; title='Désactiver les commentaires en direct' app-screenshot-end]