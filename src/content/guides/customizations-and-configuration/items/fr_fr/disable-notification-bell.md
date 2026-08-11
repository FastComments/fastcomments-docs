---
[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments affichera une cloche de notification en haut à droite de la zone de commentaires.

Cette cloche deviendra rouge et affichera un compteur du nombre de notifications que l'utilisateur a. Quelques exemples de notifications sont :

- Un utilisateur vous a répondu.
- Un utilisateur a répondu dans un fil auquel vous avez commenté.
- Un utilisateur a voté positivement votre commentaire.
- Un utilisateur a répondu à une page à laquelle vous êtes abonné.

La cloche de notification offre également un mécanisme pour s'abonner à une page entière.

Cependant, nous pouvons désactiver complètement la cloche de notification :

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Désactiver la cloche de notification'; code-example-end]

Cela peut également être fait sans code. Dans la page de personnalisation du widget, voir la section « Désactiver la cloche de notification ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Page de personnalisation du widget avec la case à cocher Désactiver la cloche de notification cochée'; title='Désactiver la cloche de notification' app-screenshot-end]

---