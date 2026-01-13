[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments affichera une cloche de notification dans le coin supérieur droit de la zone de commentaires.

Cette cloche deviendra rouge et affichera le nombre de notifications que l'utilisateur possède. Quelques exemples de notifications sont :

- Un utilisateur vous a répondu.
- Un utilisateur a répondu dans un fil où vous avez commenté.
- Un utilisateur a voté pour votre commentaire.
- Un utilisateur a répondu sur une page à laquelle vous êtes abonné.

La cloche de notification offre également un mécanisme pour s'abonner à une page entière.

Cependant, nous pouvons désactiver complètement la cloche de notification :

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Cela peut aussi être fait sans code. Dans la page de personnalisation du widget, consultez la section "Désactiver la cloche de notification".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]