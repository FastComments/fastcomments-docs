[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Pour l'authentification, FastComments dépend de l'activation des cookies tiers dans votre navigateur. Sans eux, les utilisateurs devront toujours laisser leur adresse e-mail pour commenter (à moins que le champ de saisie de l'e-mail soit masqué), et leurs commentaires apparaîtront toujours comme non vérifiés (par défaut).

Pour contourner cela, vous pouvez activer le contournement des cookies tiers. 

Lorsque ce paramètre est activé, un petit popup apparaît affichant un message indiquant que l'utilisateur est en cours de connexion. Ce popup s'affiche chaque fois que l'utilisateur interagit avec le widget de commentaires ; par exemple, lorsqu'il laisse un commentaire.

Nous pouvons le faire dans le code en définissant le flag **enableThirdPartyCookieBypass** à true :

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Nous pouvons également configurer cela via l'interface de personnalisation du widget, sous `Enable Third-Party Cookie Popup` :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---